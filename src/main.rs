use std::net::UdpSocket;

fn main() {
  let socket = UdpSocket::bind("0.0.0.0:20000").unwrap();
  socket.connect("8.8.8.8:53").unwrap();
  let query = DnsPacket::create_query("www.fresho.com", QueryType::HostAddress);

  socket.send(query.pack().as_slice()).unwrap();

  let mut buf = [0; 2048];
  socket.recv(&mut buf).unwrap();

  let response = DnsPacket::unpack(buf.to_vec());

  println!("response: {:#?}", response);
}

#[derive(PartialEq, Debug)]
enum QueryResponse {
  Query,
  Response,
}

#[derive(PartialEq, Debug)]
enum OpCode {
  Query,
}

#[derive(PartialEq, Debug)]
enum ResponseCode {
  NoError,
  FormatError,
  ServerFailure,
  NonExistentDomain,
  NotImplemented,
  Refused,
}

#[derive(PartialEq, Debug)]
struct DnsFlags {
  query_response: QueryResponse,
  op_code: OpCode,
  authoritive_answer: bool,
  truncated: bool,
  recursion_desired: bool,
  recursion_available: bool,
  reserved: bool,
  response_code: ResponseCode,
}

#[derive(PartialEq, Debug)]
struct DnsHeader {
  id: u16,
  flags: DnsFlags,
  question_count: u16,
  answer_count: u16,
  name_server_count: u16,
  additional_record_count: u16,
}

#[derive(PartialEq, Debug)]
struct Label {
  length: u8,
  octets: Vec<u8>,
}

#[derive(PartialEq, Debug)]
enum QueryType {
  HostAddress = 0x0001,
}

#[derive(PartialEq, Debug)]
enum QueryClass {
  Internet = 0x0001,
}

#[derive(PartialEq, Debug)]
struct DnsQuestion {
  query_name: Vec<Label>,
  query_type: QueryType,
  query_class: QueryClass,
}

#[derive(PartialEq, Debug)]
struct DnsAnswer {
  query_name: Vec<Label>,
  query_type: QueryType,
  query_class: QueryClass,
  time_to_live: u16,
  response_data_length: u16,
  response_data: Vec<u8>,
}

#[derive(PartialEq, Debug)]
struct DnsPacket {
  header: DnsHeader,
  questions: Vec<DnsQuestion>,
  answers: Vec<DnsAnswer>,
}
impl DnsPacket {
  fn create_query(name: &str, query_type: QueryType) -> DnsPacket {
    todo!()
  }

  fn pack(&self) -> Vec<u8> {
    todo!()
  }

  fn unpack(buf: Vec<u8>) -> DnsPacket {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use crate::*;
  use hex_literal::hex;

  #[test]
  fn test_pack() {
    let query = DnsPacket {
      header: DnsHeader {
        id: 0x9c62,
        flags: DnsFlags {
          query_response: QueryResponse::Query,
          op_code: OpCode::Query,
          authoritive_answer: false,
          truncated: false,
          recursion_desired: false,
          recursion_available: false,
          reserved: false,
          response_code: ResponseCode::NoError,
        },
        question_count: 1,
        answer_count: 0,
        name_server_count: 0,
        additional_record_count: 0,
      },
      questions: vec![DnsQuestion {
        query_name: vec![
          Label {
            length: 3,
            octets: "www".as_bytes().to_vec(),
          },
          Label {
            length: 6,
            octets: "google".as_bytes().to_vec(),
          },
          Label {
            length: 3,
            octets: "com".as_bytes().to_vec(),
          },
        ],
        query_type: QueryType::HostAddress,
        query_class: QueryClass::Internet,
      }],
      answers: vec![],
    };

    assert_eq!(
      query.pack(),
      hex!(
        "
        9c 62 01 00 00 01 00 00 00 00 00 00 03 77 77 77 
        06 67 6f 6f 67 6c 65 03 63 6f 6d 00 00 01 00 01 
        "
      )
    );
  }

  #[test]
  fn test_unpack() {
    let buf = hex!(
      "
      9c 62 01 00 00 01 00 00 00 00 00 00 03 77 77 77 
      06 67 6f 6f 67 6c 65 03 63 6f 6d 00 00 01 00 01 
      "
    );

    let query = DnsPacket {
      header: DnsHeader {
        id: 0x9c62,
        flags: DnsFlags {
          query_response: QueryResponse::Query,
          op_code: OpCode::Query,
          authoritive_answer: false,
          truncated: false,
          recursion_desired: false,
          recursion_available: false,
          reserved: false,
          response_code: ResponseCode::NoError,
        },
        question_count: 1,
        answer_count: 0,
        name_server_count: 0,
        additional_record_count: 0,
      },
      questions: vec![DnsQuestion {
        query_name: vec![
          Label {
            length: 3,
            octets: "www".as_bytes().to_vec(),
          },
          Label {
            length: 6,
            octets: "google".as_bytes().to_vec(),
          },
          Label {
            length: 3,
            octets: "com".as_bytes().to_vec(),
          },
        ],
        query_type: QueryType::HostAddress,
        query_class: QueryClass::Internet,
      }],
      answers: vec![],
    };

    assert_eq!(DnsPacket::unpack(buf.to_vec()), query,);
  }

  #[test]
  fn test_create_query() {
    let query: DnsPacket = DnsPacket::create_query("www.fresho.com", QueryType::HostAddress);
    let question = query.questions.first().unwrap();

    assert_eq!(question.query_type, QueryType::HostAddress);

    assert_eq!(
      question.query_name,
      vec![
        Label {
          length: 3,
          octets: "www".as_bytes().to_vec(),
        },
        Label {
          length: 6,
          octets: "fresho".as_bytes().to_vec(),
        },
        Label {
          length: 3,
          octets: "com".as_bytes().to_vec(),
        },
      ],
    )
  }
}
