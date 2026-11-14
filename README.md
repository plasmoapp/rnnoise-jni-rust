# rnnoise-jni-rust
Simple JNI wrapper for the [nnnoiseless](https://github.com/jneem/nnnoiseless) using [jni-rs](https://github.com/jni-rs/jni-rs).

### Adding dependency to the project
<img alt="version" src="https://img.shields.io/badge/dynamic/xml?label=%20&query=/metadata/versioning/versions/version[not(contains(text(),'%2B'))][last()]&url=https://repo.plasmoverse.com/releases/com/plasmoverse/rnnoise-jni-rust/maven-metadata.xml">

```kotlin
repositories {
    maven("https://repo.plasmoverse.com/releases")
}

dependencies {
    implementation("com.plasmoverse:rnnoise-jni-rust:$version")
}
```

### Usage
Sample code from [DenoiseTest.java](https://github.com/plasmoapp/rnnoise-jni-rust/blob/main/src/test/java/com/plasmoverse/rnnoise/DenoiseTest.java)
```java
// Creates a new RNNoise instance
Denoise denoise = Denoise.create();

// 960 is one 20 ms frame
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
denoise.close();
```
