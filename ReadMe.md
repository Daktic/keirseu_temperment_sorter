# Keirsey Temperament Sorter
This is a little project I did for fun. It's a personality test based on the Keirsey Temperament Sorter.
It's a 70 question test that will tell you what your personality type is. It's based on the Myers-Briggs Type Indicator.

## How to use
I took the liberty of pre-compiling the program for you.

All you have to do is download the zip file that matches your operating system, extract it, and run the executable file.

#### download links
- [Windows](https://github.com/Daktic/keirseu_temperment_sorter/raw/master/compiled_binaries/windows.zip)
- [Mac](https://github.com/Daktic/keirseu_temperment_sorter/raw/master/compiled_binaries/mac.zip)
- [Linux](https://github.com/Daktic/keirseu_temperment_sorter/raw/master/compiled_binaries/linux.zip)

If you would rather compile it yourself, you can do that too. You will need to download the source code and compile it with rustc.

----
## How to compile
You will need to have rustc installed on your computer. You can download it [here](https://www.rust-lang.org/tools/install).

```bash
cargo --version
```

If you get a version back, you're good to go. If not, you will need to check your rustc installation.

### Compiling it yourself
Now run the following command in the root directory of the project.

```bash
cargo build --release
```

A /target directory should be created. Inside that directory, there should be a /release directory. Inside that directory, there should be an executable file. That's the program. You can run it from there.

### Using Cargo
You can also use cargo to run the program.

```bash
cargo build
cargo run
```
