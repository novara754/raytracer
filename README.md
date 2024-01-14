# Raytracer

Multi-threaded CPU raytracer implemented in Rust following
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

![Sample image of many different spheres with different materials rendered in HD](./sample.png)

The scene in the above image rendered at 1280x720 resolution in roughly 1.5 minutes on my machine
(Ryzen R5 5600X CPU).
Each pixel is the result of 100 samples, meaning 100 rays were sent into the scene and averaged
to get the pixel color. Each ray reflected and refracted a maximum of 50 times.

The scene contains many randomly generated spheres of different materials: diffuse, metal and glass.
 - The diffuse materials have a rather soft, non-reflective surface with given colours.
 - The metal sphere also have their own base colours but reflect light depending on their fuzziness.
 - The glass balls either reflect or refract the rays that hit them depending on the angle and random chance, however they do not have any intrinsic color.

Additionally there is a depth of field effect generated by randomly, slightly offsetting the
origin of each ray slightly from the origin of the camera so that objects outside of the focal plane receive some blur. 

By default the raytracer will create a threadpool with on thread per CPU core. The pixels to be rendered
are essentially submitted to this threadpool in a queue. This means N pixels can be calculated in parallel,
where N is the number of CPU cores.
The number of threads can be changed by setting the `RAYON_NUM_THREADS` environment variable
before running the program.

## Usage

**Note:** To compile this project please install the [Rust compiler](https://rust-lang.org/).

For a debug build run `cargo build` in the project directory. For an optimized release
build please run `cargo build --release`.
The resulting binaries will be located in `./target/debug` and `./target/release` respectively.

You can also use `cargo run` or `cargo run --release` to compile and run the program in one step.

Once execution is complete the program will generate an image with the rendered scene in the current working directory. 

## License

Licensed under the [MIT License](./LICENSE).