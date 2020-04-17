
public class ClojureRust {

    private static native long getSentenceTransformerRust(String model_path, String device);
    private static native Double[][] encodeRust(long modelPtr, Object documents, int batch_size);

    public static long getSentenceTransformer(String model_path, String device) throws java.io.IOException {
        return getSentenceTransformerRust(model_path, device);
    }

    public static Double[][] encode(long modelPtr, Object documents, int batch_size) throws  java.io.IOException {
        Double [][] output = encodeRust(modelPtr, documents, batch_size);
        return output;
    }

}