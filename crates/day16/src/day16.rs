pub fn part1(input:&[String]) -> usize {
  let (_left,packet) = parse(&hex_to_binary(input));
  packet.sum_version_id()
}

pub fn part2(input:&[String]) -> usize {
  let (_left,packet) = parse(&hex_to_binary(input));
  packet.solve()
}

struct Packet {
  version_id: usize,
  type_id: usize,
  content: PacketContent 
}

impl Packet {
  fn sum_version_id(&self) -> usize {
    let mut sum = self.version_id;
    if let PacketContent::SubPackets(vector) = &self.content {
      for p in vector {
        sum+= p.sum_version_id();
      }
    }
    sum
  }
  fn solve(&self) -> usize {
    match &self.content {
      PacketContent::Number(n) => {
        *n
      }
      PacketContent::SubPackets(packets) => {
        let mut values = packets.iter().map(|p| p.solve());
        match self.type_id {
          0 => { values.sum() }
          1 => { values.product() }
          2 => { values.min().unwrap() }
          3 => { values.max().unwrap() }
          5 => { if values.next() > values.next() {1} else {0}}
          6 => { if values.next() < values.next() {1} else {0}}
          7 => { if values.next() == values.next() {1} else {0}}
          _ => panic!("Invalid type_id")
        }
      }
    }
  }
}

enum PacketContent {
  SubPackets(Vec<Packet>),
  Number(usize)
}

fn parse(input:&[u8]) -> (Vec<u8>,Packet) {
  let v_id = binary_to_usize(&[input[0],input[1],input[2]]);
  let t_id = binary_to_usize(&[input[3],input[4],input[5]]);

  let mut left = input.iter().skip(6);
  let mut bin = vec![];

  if t_id == 4 {
    // parsing for a literal value
    let mut keep_reading = true;
    while keep_reading {
      if *left.next().unwrap() == 0 {
        keep_reading = false;
      }
      for _i in 0..4{
        bin.push(*left.next().unwrap())
      }
    }
    let bin_value = binary_to_usize(&bin);
    (
      left.copied().collect(), 
      Packet {
        version_id:v_id,
        type_id:t_id,
        content:PacketContent::Number(bin_value)
      }
    )
  }
  else {
    let mut subpackets:Vec<Packet> = vec![];
    let length_type = *left.next().unwrap();
    let mut to_parse:Vec<u8>;
    if  length_type == 0 {
      // next 15 bit -> total length in bits of the subpackets here
      for _i in 0..15{
        bin.push(*left.next().unwrap())
      }
      to_parse = left.copied().collect();
      let mut bit_length = binary_to_usize(&bin);
      while bit_length!=0 {
        let starting_len = to_parse.len();

        let (to_parse_left,packet) = parse(&to_parse);
        to_parse = to_parse_left;

        bit_length -= starting_len - to_parse.len();
        subpackets.push(packet);
      }
    } else {
      // next 11 bit -> number of subpackets
      for _i in 0..11{
        bin.push(*left.next().unwrap())
      }
      to_parse = left.copied().collect();
      for _i in 0..binary_to_usize(&bin) {
        let (to_parse_left,packet) = parse(&to_parse);
        to_parse = to_parse_left;
        subpackets.push(packet);
      }
    }
    (
      to_parse.iter().copied().collect(),
      Packet {
        version_id:v_id,
        type_id:t_id,
        content:PacketContent::SubPackets(subpackets)
      }
    )
  }
}

fn hex_to_binary(input:&[String]) -> Vec<u8> {
  let mut binary_vec = vec![];
  for c in input[0].chars() {
    let binary = hex_to_bin(c);
    binary_vec.extend(binary.iter());
  }
  binary_vec
}

fn binary_to_usize(input:&[u8]) -> usize {
  let mut val = 0;
  let base:usize = 2;
  for i in 0..input.len() {
    if input[i]==1{
      val += base.pow((input.len()-i-1) as u32);
    }
  }
  val
}

fn hex_to_bin(hex:char) -> Vec<u8> {
  let num = hex.to_digit(16).unwrap();
  let v = format!("{:04b}",num);
  v.chars().map(|c| c.to_digit(2).unwrap() as u8).collect()
}


#[cfg(test)]
mod tests {
  use super::{PacketContent,parse,hex_to_binary};
  static LITERALPACKET: &str = "D2FE28";
  static OPERATORPACKET0: &str = "38006F45291200";
  static OPERATORPACKET1: &str = "EE00D40C823060";
  static OPERATORSUM1: &str = "8A004A801A8002F478";
  static OPERATORSUM2: &str = "620080001611562C8802118E34";
  static OPERATORSUM3: &str = "C0015000016115A2E0802F182340";
  static OPERATORSUM4: &str = "A0016C880162017C3686B18A3D4780";
  static OPERATORSOLVE1: &str = "C200B40A82";
  static OPERATORSOLVE2: &str = "04005AC33890";
  static OPERATORSOLVE3: &str = "880086C3E88112";
  static OPERATORSOLVE4: &str = "CE00C43D881120";
  static OPERATORSOLVE5: &str = "D8005AC2A8F0";
  static OPERATORSOLVE6: &str = "F600BC2D8F";
  static OPERATORSOLVE7: &str = "9C005AC2F8F0";
  static OPERATORSOLVE8: &str = "9C0141080250320F1802104A08";

  #[test]
  fn test_bin_to_hex() {
    let input:Vec<String> = LITERALPACKET.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let binary = hex_to_binary(&input);
    assert_eq!(vec![1,1,0,1,0,0,1,0,1,1,1,1,1,1,1,0,0,0,1,0,1,0,0,0],binary)
  }
  #[test]
  fn test_parse_literal() {
    let input:Vec<String> = LITERALPACKET.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(6,packet.version_id);
    assert_eq!(4,packet.type_id);
    if let PacketContent::Number(value) = packet.content {
      assert_eq!(2021,value);
    }
  }
  #[test]
  fn test_parse_operator_0() {
    let input:Vec<String> = OPERATORPACKET0.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(1,packet.version_id);
    assert_eq!(6,packet.type_id);
    if let PacketContent::SubPackets(vector) = packet.content {
      assert_eq!(2,vector.len());
    }
    
  }
  #[test]
  fn test_parse_operator_1() {
    let input:Vec<String> = OPERATORPACKET1.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(7,packet.version_id);
    assert_eq!(3,packet.type_id);
    if let PacketContent::SubPackets(vector) = packet.content {
      assert_eq!(3,vector.len());
    }
  }
  #[test]
  fn test_sum() {
    let input:Vec<String> = OPERATORSUM1.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(16,packet.sum_version_id());
    let input:Vec<String> = OPERATORSUM2.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(12,packet.sum_version_id());
    let input:Vec<String> = OPERATORSUM3.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(23,packet.sum_version_id());
    let input:Vec<String> = OPERATORSUM4.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(31,packet.sum_version_id())
  }
  #[test]
  fn test_op() {
    let input:Vec<String> = OPERATORSOLVE1.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(3,packet.solve());
    let input:Vec<String> = OPERATORSOLVE2.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(54,packet.solve());
    let input:Vec<String> = OPERATORSOLVE3.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(7,packet.solve());
    let input:Vec<String> = OPERATORSOLVE4.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(9,packet.solve());
    let input:Vec<String> = OPERATORSOLVE5.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(1,packet.solve());
    let input:Vec<String> = OPERATORSOLVE6.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(0,packet.solve());
    let input:Vec<String> = OPERATORSOLVE7.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(0,packet.solve());
    let input:Vec<String> = OPERATORSOLVE8.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (_left,packet) = parse(&hex_to_binary(&input));
    assert_eq!(1,packet.solve())
  }
}