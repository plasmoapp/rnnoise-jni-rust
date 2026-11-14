package com.plasmoverse.rnnoise;

import org.junit.jupiter.api.Test;

public final class DenoiseTest {

    private static final int FRAME = 960;

    @Test
    public void process() throws Exception {
        // Creates a new RNNoise instance
        Denoise denoise = Denoise.create();

        float[] input = new float[960];
        float[] output = new float[960];

        // Writes the denoised samples to output and returns the voice probability, from 0 to 1
        float probability = denoise.process(input, output);

        // Only returns the voice probability
        probability = denoise.process(input, null);

        // Returns the denoised samples in a new array
        float[] processed = denoise.process(input);

        // Clears the state, as if the instance was just created
        denoise.reset();

        // Closes the RNNoise instance, releasing allocated resources
        denoise.close();
    }
}
