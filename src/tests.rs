#[cfg(test)]
mod tests {
    use super::super::utils::disassemble;

    #[test]
    fn test_0x01() {
        let buffer: Vec<u8> = vec![0x1, 0x4, 0xd1];
        assert_eq!(disassemble(&buffer), "LXI\tB, $d14\n");
    }
    #[test]
    fn test_0x06() {
        let buffer = vec![0x6, 0x4];
        assert_eq!(disassemble(&buffer), "MVI\tB, $4\n");
    }
    #[test]
    fn test_0x0e() {
        let buffer = vec![0x0e, 0x4];
        assert_eq!(disassemble(&buffer), "MVI\tC, $4\n");
    }
    #[test]
    fn test_0x11() {
        let buffer: Vec<u8> = vec![0x11, 0x4, 0xd1];
        assert_eq!(disassemble(&buffer), "LXI\tD, $d14\n");
    }
    #[test]
    fn test_0x16() {
        let buffer = vec![0x16, 0x4];
        assert_eq!(disassemble(&buffer), "MVI\tD, $4\n");
    }
    #[test]
    fn test_0x1e() {
        let buffer = vec![0x1e, 0x10];
        assert_eq!(disassemble(&buffer), "MVI\tE, $10\n");
    }
    #[test]
    fn test_0x21() {
        let buffer = vec![0x21, 0x10, 0xd5];
        assert_eq!(disassemble(&buffer), "LXI\tH, $d510\n");
    }
    #[test]
    fn test_0x22() {
        let buffer = vec![0x22, 0x10, 0xd5];
        assert_eq!(disassemble(&buffer), "SHLD\t$d510\n");
    }
    #[test]
    fn test_0x26() {
        let buffer = vec![0x26, 0x10];
        assert_eq!(disassemble(&buffer), "MVI\tH, $10\n");
    }
    #[test]
    fn test_0x2a() {
        let buffer = vec![0x2a, 0x10, 0xd5];
        assert_eq!(disassemble(&buffer), "LHLD\t$d510\n");
    }
    #[test]
    fn test_0x2e() {
        let buffer = vec![0x2e, 0x10];
        assert_eq!(disassemble(&buffer), "MVI\tL, $10\n");
    }
    #[test]
    fn test_0x31() {
        let buffer = vec![0x31, 0x10, 0xe7];
        assert_eq!(disassemble(&buffer), "LXI\tSP, $e710\n");
    }
    #[test]
    fn test_0x32() {
        let buffer = vec![0x32, 0x10, 0xe7];
        assert_eq!(disassemble(&buffer), "STA\t$e710\n");
    }

    #[test]
    fn test_0x36() {
        let buffer = vec![0x36, 0x10];
        assert_eq!(disassemble(&buffer), "MVI\tM, $10\n");
    }
    #[test]
    fn test_0x3a() {
        let buffer = vec![0x3a, 0x10, 0xe7];
        assert_eq!(disassemble(&buffer), "LDA\t$e710\n");
    }
    #[test]
    fn test_0x3e() {
        let buffer = vec![0x3e, 0x10];
        assert_eq!(disassemble(&buffer), "MVI\tA, $10\n");
    }
}
