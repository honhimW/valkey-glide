package glide.benchmarks.clients.callback;

import glide.api.ValkeyOperator;
import glide.benchmarks.clients.AsyncClient;
import glide.benchmarks.utils.ConnectionSettings;

import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;

/**
 * @author hon_him
 * @since 2025-02-12
 */
public class ValkeyAsyncClient implements AsyncClient<String> {

    private int idx = 0;

    private final List<ValkeyOperator> valkeyOperators = new CopyOnWriteArrayList<>();

    private ValkeyOperator next() {
        ValkeyOperator operator;
        int index = idx;
        if (index < 32) {
            operator = valkeyOperators.get(index);
        } else {
            operator = valkeyOperators.get(0);
        }
        idx++;
        return operator;
    }

    @Override
    public Future<String> asyncSet(String key, String value) {
        return next().set(key, value);
    }

    @Override
    public Future<String> asyncGet(String key) {
        return next().get(key);
    }

    @Override
    public void connectToValkey(ConnectionSettings connectionSettings) {
        String url =
            String.format(
                "%s://%s:%d",
                connectionSettings.useSsl ? "rediss" : "redis",
                connectionSettings.host,
                connectionSettings.port);
        for (int i = 0; i < 32; i++) {
            ValkeyOperator operator = ValkeyOperator.fromUrl(url);
            CompletableFuture<Void> start = operator.start();
            try {
                start.get(1, TimeUnit.SECONDS);
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
            valkeyOperators.add(operator);
        }
    }

    @Override
    public String getName() {
        return "glide-callback";
    }
}
