package com.plasmoverse.rnnoise;

import java.io.IOException;

/**
 * <p>
 *     RNNoise keeps state between frames, so one instance is one mono stream.
 *     Use a separate instance per channel, and call {@link #reset()} when the stream restarts.
 * </p>
 *
 * <p>
 *     Instances are not thread-safe, and must not be used after {@link #close()}.
 * </p>
 */
public final class Denoise {

    /**
     * Creates a new native RNNoise instance.
     *
     * @throws IOException If an error occurs while extracting the native library.
     * @throws UnsatisfiedLinkError If the native library fails to load.
     * @throws DenoiseException If RNNoise fails to initialize.
     * @return An instance of RNNoise.
     */
    public static Denoise create() throws IOException, DenoiseException {
        DenoiseLibrary.load();

        long pointer = createNative();

        return new Denoise(pointer);
    }

    private static native long createNative();


    private long pointer;

    private Denoise(long pointer) {
        this.pointer = pointer;
    }

    /**
     * Denoises the samples and returns the probability that they contain voice.
     *
     * @param input Mono 48 kHz samples in the 16-bit range, a positive multiple of 480 samples (10 ms) long.
     * @param output The array to write the denoised samples to, the same length as {@code input},
     *               or {@code null} if only the probability is needed.
     *               It can be the same array as {@code input}.
     * @return The highest voice probability, from 0 to 1, among the 10 ms frames in {@code input}.
     * Values above 0.5 can generally be considered voice.
     * @throws DenoiseException If the instance is closed, {@code input} isn't a positive multiple of 480 samples long,
     * or {@code output} has a different length.
     */
    public float process(float[] input, float[] output) throws DenoiseException {
        if (!isOpen()) throw new DenoiseException("Denoise is closed");

        return processNative(input, output);
    }

    /**
     * Denoises the samples into a new array.
     *
     * @param input Mono 48 kHz samples in the 16-bit range, a positive multiple of 480 samples (10 ms) long.
     * @return The denoised samples, the same length as {@code input}.
     * @throws DenoiseException If the instance is closed or {@code input} isn't a positive multiple of 480 samples long.
     * @see #process(float[], float[])
     */
    public float[] process(float[] input) throws DenoiseException {
        float[] output = new float[input.length];
        process(input, output);

        return output;
    }

    /**
     * Clears RNNoise's state, as if the instance was just created.
     *
     * @throws DenoiseException If the instance is closed.
     */
    public void reset() throws DenoiseException {
        if (!isOpen()) throw new DenoiseException("Denoise is closed");

        resetNative();
    }

    /**
     * Closes the RNNoise instance, releasing any allocated resources.
     */
    public void close() {
        if (!isOpen()) return;

        closeNative();
    }

    /**
     * Checks if the RNNoise instance is open and ready for processing.
     *
     * @return {@code true} if the instance is open, {@code false} otherwise.
     */
    public boolean isOpen() {
        return pointer != 0;
    }

    private native float processNative(float[] input, float[] output);

    private native void resetNative();

    private native void closeNative();
}
