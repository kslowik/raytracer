# raytracer

### Edit: zauważyłem, że coś się zepsuło w repo po tym jak wczoraj tam wrzuciłem wszystko, więc zrobiłem nowe

## Things added:

- Ray tracing features from "Ray Tracing in One Weekend"
- Parallel rendering using CPU
- Changed format to PNG
- Creating new scenes by JSON description

## Things to do:

- user interface
- GPU parallelization
- different objects
- more raytracing features (e.g. texture mapping)
- maybe animations?

## Usage
```
$ cargo build --release
$ ./target/release/raytracer data/example_scene.json picture.png
```
I recommend trying to create picture from impressive_scene.json and/or trying to make your own scene.
