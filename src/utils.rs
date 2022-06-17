// print opcode name and corresponding bytes according to size
pub fn process_opcode(
    buffer: &Vec<u8>,
    pc: &mut usize,
    output: &mut String,
    opcode: &str,
    size: i32,
) {
    match size {
        1 => {
            output.push_str(&format!("{}\n", opcode));
        }
        2 => output.push_str(&format!(
            "{}${}\n",
            opcode,
            format!("{:x}", buffer[*pc + 1])
        )),
        3 => output.push_str(&format!(
            "{}${}{}\n",
            opcode,
            format!("{:x}", buffer[*pc + 2]),
            format!("{:x}", buffer[*pc + 1])
        )),
        _ => {
            panic!("Undefined Opcode")
        }
    }
    *pc += size as usize;
}

pub fn disassemble(buffer: &Vec<u8>) -> String {
    let mut pc = 0; // program counter
    let mut output_string = String::new(); //disassembly

    while pc < buffer.len() {
        match buffer[pc] {
            0x00 => process_opcode(buffer, &mut pc, &mut output_string, "NOP", 1),
            0x01 => process_opcode(buffer, &mut pc, &mut output_string, "LXI\tB, ", 3),
            0x02 => process_opcode(buffer, &mut pc, &mut output_string, "STAX\tB", 1),
            0x03 => process_opcode(buffer, &mut pc, &mut output_string, "INX\tB", 1),
            0x04 => process_opcode(buffer, &mut pc, &mut output_string, "INR\tB", 1),
            0x05 => process_opcode(buffer, &mut pc, &mut output_string, "DCR\tB", 1),
            0x06 => process_opcode(buffer, &mut pc, &mut output_string, "MVI\tB, ", 2),
            0x07 => process_opcode(buffer, &mut pc, &mut output_string, "RLC", 1),
            0x09 => process_opcode(buffer, &mut pc, &mut output_string, "DAD\tB", 1),
            0x0a => process_opcode(buffer, &mut pc, &mut output_string, "LDAX\tB", 1),
            0x0b => process_opcode(buffer, &mut pc, &mut output_string, "DCX\tB", 1),
            0x0c => process_opcode(buffer, &mut pc, &mut output_string, "INR\tC", 1),
            0x0d => process_opcode(buffer, &mut pc, &mut output_string, "DCR\tC", 1),
            0x0e => process_opcode(buffer, &mut pc, &mut output_string, "MVI\tC, ", 2),
            0x0f => process_opcode(buffer, &mut pc, &mut output_string, "RRC", 1),

            0x11 => process_opcode(buffer, &mut pc, &mut output_string, "LXI\tD, ", 3),
            0x12 => process_opcode(buffer, &mut pc, &mut output_string, "STAX\tD", 1),
            0x13 => process_opcode(buffer, &mut pc, &mut output_string, "INX\tD", 1),
            0x14 => process_opcode(buffer, &mut pc, &mut output_string, "INR\tD", 1),
            0x15 => process_opcode(buffer, &mut pc, &mut output_string, "DCR\tD", 1),
            0x16 => process_opcode(buffer, &mut pc, &mut output_string, "MVI\tD, ", 2),
            0x17 => process_opcode(buffer, &mut pc, &mut output_string, "RAL", 1),
            0x19 => process_opcode(buffer, &mut pc, &mut output_string, "DAD\tD", 1),
            0x1a => process_opcode(buffer, &mut pc, &mut output_string, "LDAX\tD", 1),
            0x1b => process_opcode(buffer, &mut pc, &mut output_string, "DCX\tD", 1),
            0x1c => process_opcode(buffer, &mut pc, &mut output_string, "INR\tE", 1),
            0x1d => process_opcode(buffer, &mut pc, &mut output_string, "DCR\tE", 1),
            0x1e => process_opcode(buffer, &mut pc, &mut output_string, "MVI\tE, ", 2),
            0x1f => process_opcode(buffer, &mut pc, &mut output_string, "RAR", 1),

            0x21 => process_opcode(buffer, &mut pc, &mut output_string, "LXI\tH, ", 3),
            0x22 => process_opcode(buffer, &mut pc, &mut output_string, "SHLD\t", 3),
            0x23 => process_opcode(buffer, &mut pc, &mut output_string, "INX\tH", 1),
            0x24 => process_opcode(buffer, &mut pc, &mut output_string, "INR\tH", 1),
            0x25 => process_opcode(buffer, &mut pc, &mut output_string, "DCR\tH", 1),
            0x26 => process_opcode(buffer, &mut pc, &mut output_string, "MVI\tH, ", 2),
            0x27 => process_opcode(buffer, &mut pc, &mut output_string, "DAA", 1),
            0x29 => process_opcode(buffer, &mut pc, &mut output_string, "DAD\tH", 1),
            0x2a => process_opcode(buffer, &mut pc, &mut output_string, "LHLD\t", 3),
            0x2b => process_opcode(buffer, &mut pc, &mut output_string, "DCX\tH", 1),
            0x2c => process_opcode(buffer, &mut pc, &mut output_string, "INR\tL", 1),
            0x2d => process_opcode(buffer, &mut pc, &mut output_string, "DCR\tL", 1),
            0x2e => process_opcode(buffer, &mut pc, &mut output_string, "MVI\tL, ", 2),
            0x2f => process_opcode(buffer, &mut pc, &mut output_string, "CMA", 1),

            0x31 => process_opcode(buffer, &mut pc, &mut output_string, "LXI\tSP, ", 3),
            0x32 => process_opcode(buffer, &mut pc, &mut output_string, "STA\t", 3),
            0x33 => process_opcode(buffer, &mut pc, &mut output_string, "INX\tSP", 1),
            0x34 => process_opcode(buffer, &mut pc, &mut output_string, "INR\tM", 1),
            0x35 => process_opcode(buffer, &mut pc, &mut output_string, "DCR\tM", 1),
            0x36 => process_opcode(buffer, &mut pc, &mut output_string, "MVI\tM, ", 2),
            0x37 => process_opcode(buffer, &mut pc, &mut output_string, "STC", 1),
            0x39 => process_opcode(buffer, &mut pc, &mut output_string, "DAD\tSP", 1),
            0x3a => process_opcode(buffer, &mut pc, &mut output_string, "LDA\t", 3),
            0x3b => process_opcode(buffer, &mut pc, &mut output_string, "DCX\tSP", 1),
            0x3c => process_opcode(buffer, &mut pc, &mut output_string, "INR\tA", 1),
            0x3d => process_opcode(buffer, &mut pc, &mut output_string, "DCR\tA", 1),
            0x3e => process_opcode(buffer, &mut pc, &mut output_string, "MVI\tA, ", 2),
            0x3f => process_opcode(buffer, &mut pc, &mut output_string, "CMC", 1),

            0x40 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tB, B", 1),
            0x41 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tB, C", 1),
            0x42 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tB, D", 1),
            0x43 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tB, E", 1),
            0x44 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tB, H", 1),
            0x45 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tB, L", 1),
            0x46 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tB, M", 1),
            0x47 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tB, A", 1),
            0x48 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tC, B", 1),
            0x49 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tC, C", 1),
            0x4a => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tC, D", 1),
            0x4b => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tC, E", 1),
            0x4c => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tC, H", 1),
            0x4d => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tC, L", 1),
            0x4e => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tC, M", 1),
            0x4f => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tC, A", 1),

            0x50 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tD, B", 1),
            0x51 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tD, C", 1),
            0x52 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tD, D", 1),
            0x53 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tD, E", 1),
            0x54 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tD, H", 1),
            0x55 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tD, L", 1),
            0x56 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tD, M", 1),
            0x57 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tD, A", 1),
            0x58 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tE, B", 1),
            0x59 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tE, C", 1),
            0x5a => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tE, D", 1),
            0x5b => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tE, E", 1),
            0x5c => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tE, H", 1),
            0x5d => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tE, L", 1),
            0x5e => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tE, M", 1),
            0x5f => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tE, A", 1),


            0x60 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tH, B", 1),
            0x61 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tH, C", 1),
            0x62 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tH, D", 1),
            0x63 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tH, E", 1),
            0x64 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tH, H", 1),
            0x65 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tH, L", 1),
            0x66 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tH, M", 1),
            0x67 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tH, A", 1),
            0x68 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tL, B", 1),
            0x69 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tL, C", 1),
            0x6a => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tL, D", 1),
            0x6b => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tL, E", 1),
            0x6c => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tL, H", 1),
            0x6d => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tL, L", 1),
            0x6e => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tL, M", 1),
            0x6f => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tL, A", 1),

            0x70 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tM, B", 1),
            0x71 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tM, C", 1),
            0x72 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tM, D", 1),
            0x73 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tM, E", 1),
            0x74 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tM, H", 1),
            0x75 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tM, L", 1),
            0x76 => process_opcode(buffer, &mut pc, &mut output_string, "HLT", 1),
            0x77 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tM, A", 1),
            0x78 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tA, B", 1),
            0x79 => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tA, C", 1),
            0x7a => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tA, D", 1),
            0x7b => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tA, E", 1),
            0x7c => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tA, H", 1),
            0x7d => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tA, L", 1),
            0x7e => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tA, M", 1),
            0x7f => process_opcode(buffer, &mut pc, &mut output_string, "MOV\tA, A", 1),

            
            

            0xc3 => process_opcode(buffer, &mut pc, &mut output_string, "JMP\t", 3),
            _ => {
                //println!("Yet to be Implemented");
                //pc += 1;
                process_opcode(
                    buffer,
                    &mut pc,
                    &mut output_string,
                    "Yet to be Implemented",
                    1,
                );
            }
        }
    }

    output_string
}
