# MP4 Inspector

MP4 Inspector is a robust and versatile tool designed to analyze and parse MP4 files. Built with Rust, it provides detailed insights into MP4 containers, including their structure, box types, and atom details. Whether you're a developer working with video data or a researcher investigating MP4 file formats, MP4 Inspector offers an efficient way to inspect and understand your video files.

## 🚀 Features

- **Detailed Atom Analysis:** Uncover and understand the structure of MP4 files by parsing atoms like `stss`, `stco`, and `stsz`.
- **Dynamic Parsing:** Efficiently handles various box types, including complex container boxes.
- **Custom Skip Logic:** Tailor parsing behavior for specific atom types with extendable skip bytes configuration.
- **Human-Readable Output:** View parsed information in a clear, hierarchical format with insightful details.

## 📦 Installation

To get started with MP4 Box Explorer, follow these steps:

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/your-username/mp4-box-explorer.git

2. **Navigate to the Project Directory:**
    ```bash
    cd mp4-box-explorer

3. **Build the Project:**
    Ensure you have Rust installed. If not, you can get it from rust-lang.org. Then run:
    ```bash
    cargo build --release
    
4. **Run the Tool:**
   ```bash
   ./target/release/mp4-box-explorer

  Replace sample.mp4 with the path to your MP4 file in the main function of src/main.rs.

## 🛠 Example Output
       ```bash
       [ftyp] @ 0x00000000 of size: 28, ends @ 0x0000001c
       [wide] @ 0x0000001c of size: 8, ends @ 0x00000024
       [mdat] @ 0x00000024 of size: 1248415844, ends @ 0x4a695088
       [moov] @ 0x4a695088 of size: 28354, ends @ 0x4a69bf4a
           [mvhd] @ 0x4a695090 of size: 108, ends @ 0x4a6950fc
           [udta] @ 0x4a6950fc of size: 2048, ends @ 0x4a6958fc
               [�xyz] @ 0x4a695104 of size: 29, ends @ 0x4a695121
               [�xsp] @ 0x4a695121 of size: 18, ends @ 0x4a695133
               [�ysp] @ 0x4a695133 of size: 18, ends @ 0x4a695145
               [�zsp] @ 0x4a695145 of size: 18, ends @ 0x4a695157
               [�fpt] @ 0x4a695157 of size: 18, ends @ 0x4a695169
               [�fyw] @ 0x4a695169 of size: 20, ends @ 0x4a69517d
               [�frl] @ 0x4a69517d of size: 18, ends @ 0x4a69518f
               [�gpt] @ 0x4a69518f of size: 18, ends @ 0x4a6951a1
               [�gyw] @ 0x4a6951a1 of size: 18, ends @ 0x4a6951b3
               [�grl] @ 0x4a6951b3 of size: 18, ends @ 0x4a6951c5
               [�rec] @ 0x4a6951c5 of size: 20, ends @ 0x4a6951d9
               [�res] @ 0x4a6951d9 of size: 32, ends @ 0x4a6951f9
               [meta] @ 0x4a6951f9 of size: 256, ends @ 0x4a6952f9
                   [hdlr] @ 0x4a695205 of size: 32, ends @ 0x4a695225
                   [ilst] @ 0x4a695225 of size: 212, ends @ 0x4a6952f9
                       [�cmt] @ 0x4a69522d of size: 204, ends @ 0x4a6952f9
                           [data] @ 0x4a695235 of size: 196, ends @ 0x4a6952f9
               [Xtra] @ 0x4a6952f9 of size: 256, ends @ 0x4a6953f9
               [�uid] @ 0x4a6953f9 of size: 16, ends @ 0x4a695409
               [�mdl] @ 0x4a695409 of size: 32, ends @ 0x4a695429
               [�csn] @ 0x4a695429 of size: 48, ends @ 0x4a695459
               [�aud] @ 0x4a695459 of size: 40, ends @ 0x4a695481
               [�mux] @ 0x4a695481 of size: 128, ends @ 0x4a695501
               [�mdt] @ 0x4a695501 of size: 40, ends @ 0x4a695529
               [free] @ 0x4a695529 of size: 979, ends @ 0x4a6958fc
           [trak] @ 0x4a6958fc of size: 24965, ends @ 0x4a69ba81
               [tkhd] @ 0x4a695904 of size: 92, ends @ 0x4a695960
               [edts] @ 0x4a695960 of size: 36, ends @ 0x4a695984
                   [elst] @ 0x4a695968 of size: 28, ends @ 0x4a695984
               [mdia] @ 0x4a695984 of size: 24829, ends @ 0x4a69ba81
                   [mdhd] @ 0x4a69598c of size: 32, ends @ 0x4a6959ac
                   [hdlr] @ 0x4a6959ac of size: 49, ends @ 0x4a6959dd
                   [minf] @ 0x4a6959dd of size: 24740, ends @ 0x4a69ba81
                       [vmhd] @ 0x4a6959e5 of size: 20, ends @ 0x4a6959f9
                       [dinf] @ 0x4a6959f9 of size: 36, ends @ 0x4a695a1d
                           [dref] @ 0x4a695a01 of size: 28, ends @ 0x4a695a1d
                               [alis] @ 0x4a695a11 of size: 12, ends @ 0x4a695a1d
                       [stbl] @ 0x4a695a1d of size: 24676, ends @ 0x4a69ba81
                           [stsd] @ 0x4a695a25 of size: 180, ends @ 0x4a695ad9
                               [avc1] @ 0x4a695a35 of size: 164, ends @ 0x4a695ad9
                                   [avcC] @ 0x4a695a8b of size: 78, ends @ 0x4a695ad9
                           [stts] @ 0x4a695ad9 of size: 24, ends @ 0x4a695af1
                           [ctts] @ 0x4a695af1 of size: 24, ends @ 0x4a695b09
                           [stsc] @ 0x4a695b09 of size: 28, ends @ 0x4a695b25
                           [stsz] @ 0x4a695b25 of size: 12000, ends @ 0x4a698a05
                              Parsing stsz box...
                                  Version: 0, Flags: [00, 00, 00], Sample Size: 0, Number of Entries: 2995
                                  Entry Size: 2452032
                                  Entry Size: 968692
                                  Entry Size: 946459
                                  ...
                                  Entry Size: 387535
                           [stco] @ 0x4a698a05 of size: 11996, ends @ 0x4a69b8e1
                              Parsing stco box...
                                  Version: 0, Flags: [00, 00, 00], Number of Entries: 2995
                                  Chunk Offset: 0x00023d42
                                  Chunk Offset: 0x0027a782
                                  Chunk Offset: 0x00366f76
                                  ...
                                  Chunk Offset: 0x4a6366b9
                           [stss] @ 0x4a69b8e1 of size: 416, ends @ 0x4a69ba81
                              Parsing stss box...
                                  Version: 0, Flags: [00, 00, 00], Number of Entries: 100
                                  Sample Number: 1
                                  Sample Number: 31
                                  Sample Number: 61
                                  ...
                                  Sample Number: 2971
           [trak] @ 0x4a69ba81 of size: 1225, ends @ 0x4a69bf4a
               [tkhd] @ 0x4a69ba89 of size: 92, ends @ 0x4a69bae5
               [mdia] @ 0x4a69bae5 of size: 1125, ends @ 0x4a69bf4a
                   [mdhd] @ 0x4a69baed of size: 32, ends @ 0x4a69bb0d
                   [hdlr] @ 0x4a69bb0d of size: 49, ends @ 0x4a69bb3e
                   [minf] @ 0x4a69bb3e of size: 1036, ends @ 0x4a69bf4a
                       [gmhd] @ 0x4a69bb46 of size: 56, ends @ 0x4a69bb7e
                           [gmin] @ 0x4a69bb4e of size: 24, ends @ 0x4a69bb66
                           [priv] @ 0x4a69bb66 of size: 24, ends @ 0x4a69bb7e
                       [dinf] @ 0x4a69bb7e of size: 36, ends @ 0x4a69bba2
                           [dref] @ 0x4a69bb86 of size: 28, ends @ 0x4a69bba2
                               [alis] @ 0x4a69bb96 of size: 12, ends @ 0x4a69bba2
                       [stbl] @ 0x4a69bba2 of size: 936, ends @ 0x4a69bf4a
                           [stsd] @ 0x4a69bbaa of size: 32, ends @ 0x4a69bbca
                               [priv] @ 0x4a69bbba of size: 16, ends @ 0x4a69bbca
                           [stts] @ 0x4a69bbca of size: 24, ends @ 0x4a69bbe2
                           [stsc] @ 0x4a69bbe2 of size: 28, ends @ 0x4a69bbfe
                           [stsz] @ 0x4a69bbfe of size: 424, ends @ 0x4a69bda6
                              Parsing stsz box...
                                  Version: 0, Flags: [00, 00, 00], Sample Size: 0, Number of Entries: 101
                                  Entry Size: 146460
                                  Entry Size: 250
                                  Entry Size: 250
                                  ...
                                  Entry Size: 250
                           [stco] @ 0x4a69bda6 of size: 420, ends @ 0x4a69bf4a
                              Parsing stco box...
                                  Version: 0, Flags: [00, 00, 00], Number of Entries: 101
                                  Chunk Offset: 0x0000002c
                                  Chunk Offset: 0x00023c48
                                  Chunk Offset: 0x00c4d18b
                                  ...
                                  Chunk Offset: 0x49c8478b
      Total number of atoms parsed: 72

## 📝 Contributing
Contributions to MP4 Inspector are welcome! If you have suggestions, bug reports, or improvements, please submit an issue or a pull request.
    1. Fork the repository.
    2. Create a new branch (git checkout -b feature-branch).
    3. Commit your changes (git commit -am 'Add new feature').
    4. Push to the branch (git push origin feature-branch).
    5. Create a new Pull Request.

## 📄 License
This project is licensed under the MIT License. See the LICENSE file for details.
