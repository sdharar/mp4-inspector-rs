/*
 * Copyright (c) 2024 [Your Name or Your Organization]
 * All rights reserved.
 *
 * This code is provided "as is" without any warranty of any kind.
 * You may use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, subject to the following conditions:
 *
 * 1. The above copyright notice and this permission notice must be included
 *    in all copies or substantial portions of the Software.
 *
 * 2. The Software is provided "as is", without warranty of any kind.
 *    For more details, refer to the LICENSE file included with this code.
 */

 use std::fs::File;
 use std::io::{self, Read, Seek, SeekFrom};
 use byteorder::{BigEndian, ReadBytesExt};
 
 /// Represents the header of a box in the MP4 file.
 #[derive(Debug)]
 struct BoxHeader {
     size: u32,
     box_type: [u8; 4],
     start: u64,
     end: u64,
 }
 
 impl BoxHeader {
     /// Determines if the box is a container type that may have sub-boxes.
     fn is_container(&self) -> bool {
         matches!(
             &self.box_type,
             b"moov"  | b"ilst" | b"trak" | b"meta" | b"udta" | b"stsd" | b"dref" | b"edts" | b"mdia" | b"minf" | b"gmhd" | b"dinf"  | b"\xA9cmt" | b"stbl" | b"avc1" | b"mvex" | b"iods" | b"moof" | b"traf" | b"mfra" | b"skip" | b"strk" | b"ipro" | b"fiin" | b"paen" | b"meco" | b"mere"
         )
     }
 
     /// Returns the number of bytes to skip based on the box type.
     fn get_skip_bytes(&self) -> u64 {
         match &self.box_type {
             b"avc1" => 86 - 8, // Example: skip 86 bytes for `avc1` minus 8 bytes of header
             b"meta" => 4,      // Example: skip 4 bytes for `meta`
             _ => 0,
         }
     }
 }
 
 /// Reads the header of a box from the given reader.
 fn read_box_header<R: Read + Seek>(reader: &mut R) -> io::Result<BoxHeader> {
     let start = reader.seek(SeekFrom::Current(0))?;
     let size = reader.read_u32::<BigEndian>()?;
     let mut box_type = [0u8; 4];
     reader.read_exact(&mut box_type)?;
 
     let end = if size == 0 {
         start + 8 // Only the header size for zero-sized atoms
     } else {
         start + size as u64
     };
 
     Ok(BoxHeader {
         size,
         box_type,
         start,
         end,
     })
 }
 
 /// Prints information about a box, including its type, start position, size, and end position.
 fn print_box_info(header: &BoxHeader, depth: usize) {
     let indent = "    ".repeat(depth);
 
     if header.size != 0 {
         println!(
             "{} [{}] @ {:#010x} of size: {}, ends @ {:#010x}",
             indent,
             String::from_utf8_lossy(&header.box_type),
             header.start,
             header.size,
             header.end
         );
     }
 }
 
 /// Skips over the data of a box based on its size.
 fn skip_box_data<R: Read + Seek>(reader: &mut R, size: u32) -> io::Result<()> {
     let bytes_to_skip = (size - 8) as i64;
     reader.seek(SeekFrom::Current(bytes_to_skip))?;
     Ok(())
 }
 
 /// Parses the `stss` box, which contains sample sync information.
 fn parse_stss<R: Read + Seek>(reader: &mut R, _size: u32, depth: usize) -> io::Result<()> {
     println!("{}Parsing stss box...", "    ".repeat(depth));
     let version = reader.read_u8()?;
     let mut flags = [0u8; 3];
     reader.read_exact(&mut flags)?;
 
     let num_entries = reader.read_u32::<BigEndian>()?;
     println!("{}    Version: {}, Flags: {:02x?}, Number of Entries: {}", "    ".repeat(depth), version, flags, num_entries);
 
     let mut sample_numbers = Vec::with_capacity(num_entries as usize);
     for _ in 0..num_entries {
         let sample_number = reader.read_u32::<BigEndian>()?;
         sample_numbers.push(sample_number);
     }
 
     // Print first 3 entries and the last one
     let display_count = 3;
     if sample_numbers.len() > display_count {
         for &number in sample_numbers.iter().take(display_count) {
             println!("{}    Sample Number: {}", "    ".repeat(depth), number);
         }
         println!("{}    ...", "    ".repeat(depth));
         println!("{}    Sample Number: {}", "    ".repeat(depth), sample_numbers.last().unwrap());
     } else {
         for number in sample_numbers {
             println!("{}    Sample Number: {}", "    ".repeat(depth), number);
         }
     }
 
     Ok(())
 }
 
 /// Parses the `stco` box, which contains chunk offset information.
 fn parse_stco<R: Read + Seek>(reader: &mut R, _size: u32, depth: usize) -> io::Result<()> {
     println!("{}Parsing stco box...", "    ".repeat(depth));
     let version = reader.read_u8()?;
     let mut flags = [0u8; 3];
     reader.read_exact(&mut flags)?;
 
     let num_entries = reader.read_u32::<BigEndian>()?;
     println!("{}    Version: {}, Flags: {:02x?}, Number of Entries: {}", "    ".repeat(depth), version, flags, num_entries);
 
     let mut chunk_offsets = Vec::with_capacity(num_entries as usize);
     for _ in 0..num_entries {
         let chunk_offset = reader.read_u32::<BigEndian>()?;
         chunk_offsets.push(chunk_offset);
     }
 
     // Print first 3 entries and the last one in hexadecimal format
     let display_count = 3;
     if chunk_offsets.len() > display_count {
         for &offset in chunk_offsets.iter().take(display_count) {
             println!("{}    Chunk Offset: {:#010x}", "    ".repeat(depth), offset);
         }
         println!("{}    ...", "    ".repeat(depth));
         println!("{}    Chunk Offset: {:#010x}", "    ".repeat(depth), chunk_offsets.last().unwrap());
     } else {
         for offset in chunk_offsets {
             println!("{}    Chunk Offset: {:#010x}", "    ".repeat(depth), offset);
         }
     }
 
     Ok(())
 }
 
 /// Parses the `stsz` box, which contains sample size information.
 fn parse_stsz<R: Read + Seek>(reader: &mut R, _size: u32, depth: usize) -> io::Result<()> {
     println!("{}Parsing stsz box...", "    ".repeat(depth));
     let version = reader.read_u8()?;
     let mut flags = [0u8; 3];
     reader.read_exact(&mut flags)?;
 
     let sample_size = reader.read_u32::<BigEndian>()?;
     let num_entries = reader.read_u32::<BigEndian>()?;
     println!("{}    Version: {}, Flags: {:02x?}, Sample Size: {}, Number of Entries: {}", "    ".repeat(depth), version, flags, sample_size, num_entries);
 
     if sample_size == 0 {
         let mut entry_sizes = Vec::with_capacity(num_entries as usize);
         for _ in 0..num_entries {
             let entry_size = reader.read_u32::<BigEndian>()?;
             entry_sizes.push(entry_size);
         }
 
         // Print first 3 entries and the last one
         let display_count = 3;
         if entry_sizes.len() > display_count {
             for &size in entry_sizes.iter().take(display_count) {
                 println!("{}    Entry Size: {}", "    ".repeat(depth), size);
             }
             println!("{}    ...", "    ".repeat(depth));
             println!("{}    Entry Size: {}", "    ".repeat(depth), entry_sizes.last().unwrap());
         } else {
             for size in entry_sizes {
                 println!("{}    Entry Size: {}", "    ".repeat(depth), size);
             }
         }
     }
 
     Ok(())
 }
 
 /// Recursively parses boxes and sub-boxes within the MP4 file.
 fn parse_boxes<R: Read + Seek>(reader: &mut R, end: u64, depth: usize, counter: &mut usize) -> io::Result<()> {
     while reader.seek(SeekFrom::Current(0))? < end {
         let header = read_box_header(reader)?;
 
         // Check if the header end exceeds the parent end
         if header.end > end {
             reader.seek(SeekFrom::Current(8))?;
             continue;
         }
 
         // Print the box info with the current depth
         print_box_info(&header, depth);
 
         // Increment the counter for each parsed atom
         *counter += 1;
 
         // Check if specific skipping is needed
         let skip_bytes = header.get_skip_bytes();
         if skip_bytes > 0 {
             // Seek to the new position after skipping
             reader.seek(SeekFrom::Current(skip_bytes as i64))?;
             // Increase depth before parsing sub-boxes
             if header.is_container() {
                 parse_boxes(reader, header.end, depth + 1, counter)?;
             }
             // Ensure to skip any additional bytes if necessary
             continue;
         }
 
         // If the box size is zero, skip this box.
         if header.size == 0 {
             reader.seek(SeekFrom::Start(header.end))?;
             *counter -= 1;
             continue;
         }
 
         // Prepare to parse sub-boxes
         let box_size = header.size as u64;
         let next_box_pos = reader.seek(SeekFrom::Current(0))? + box_size - 8;
 
         if header.is_container() {
             // Increase depth before parsing sub-boxes
             parse_boxes(reader, header.end, depth + 1, counter)?;
         } else {
             // Call specific parser functions for stss, stco, and stsz atoms
             match &header.box_type {
                 b"stss" => parse_stss(reader, header.size, depth + 1)?,
                 b"stco" => parse_stco(reader, header.size, depth + 1)?,
                 b"stsz" => parse_stsz(reader, header.size, depth + 1)?,
                 _ => skip_box_data(reader, header.size)?,
             }
         }
 
         // Ensure the next position is within bounds
         if next_box_pos > end {
             break;
         }
         reader.seek(SeekFrom::Start(header.end))?;
     }
     Ok(())
 }
 
 /// Main function to execute the MP4 parsing.
 fn main() -> io::Result<()> {
     let file_path = "sample.mp4";
     let mut file = match File::open(file_path) {
         Ok(file) => file,
         Err(e) => {
             eprintln!("Error opening file {}: {}", file_path, e);
             return Err(e);
         }
     };
 
     let file_size = file.seek(SeekFrom::End(0))?;
     file.seek(SeekFrom::Start(0))?;
 
     let mut counter = 0;
     parse_boxes(&mut file, file_size, 0, &mut counter)?;
 
     println!("Total number of atoms parsed: {}", counter);
 
     Ok(())
 }
 