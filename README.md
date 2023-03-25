# one-for-all-amiibo

The one-for-all-amiibo tool allows you to easily merge multiple amiibo bin files into a single file. This can be useful if you have multiple amiibos that you want to use with an emulator or other software that supports amiibo files.

## Installation

You can download the latest release from the [releases page](https://github.com/example/one-for-all-amiibo/releases) on GitHub. There are pre-built binaries available for Windows, macOS, and Linux. Simply download the appropriate binary for your operating system and extract the archive.

## Usage

To use one-for-all-amiibo, simply run the binary and pass the paths of the input files as arguments. For example:


./one-for-all-amiibo input1.bin input2.bin input3.bin output.bin


This command will combine the data from `input1.bin`, `input2.bin`, and `input3.bin`, and write the resulting data to `output.bin`.

By default, each input file is padded with zeros to a size of 572 bytes, which is the size of a typical amiibo bin file. If you want to specify a different size, you can use the `--pad-size` flag followed by the desired size in bytes. For example:


./one-for-all-amiibo –pad-size 540 input1.bin input2.bin output.bin


This command will pad each input file with zeros to a size of 540 bytes before combining them.

## Development

If you want to build one-for-all-amiibo from source, you will need to have Rust installed. You can then clone the repository and run the following command:


cargo build –release


This will build the binary in release mode, which is optimized for performance.

## License

one-for-all-amiibo is released under the [MIT License](https://github.com/example/one-for-all-amiibo/blob/main/LICENSE).

## Contributing

Contributions are welcome! If you want to contribute to one-for-all-amiibo, please read the [contributing guidelines](https://github.com/example/one-for-all-amiibo/blob/main/CONTRIBUTING.md) before submitting a pull request.

