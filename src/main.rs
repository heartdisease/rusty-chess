use std::fmt;
use std::io::{self, Read};

mod rustychess {
  #[derive(Clone, Copy, PartialEq)]
  pub enum Player {
    White,
    Black,
    None,
  }

  #[derive(Clone, Copy, PartialEq)]
  pub enum Token {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
  }

  #[derive(Clone, Copy)]
  struct Field {
    token: Token,
    player: Player,
  }

  impl Field {
    fn new() -> Field {
      Field {
        token: Token::None,
        player: Player::None,
      }
    }

    fn swap(&mut self, field: &mut Field) {
      self.token = field.token;
      self.player = field.player;
      field.clear();
    }

    fn clear(&mut self) {
      self.token = Token::None;
      self.player = Player::None;
    }
  }

  pub struct Chessboard {
    fields: [[Field; 8]; 8],
    perspective: Player,
  }

  impl Chessboard {
    pub fn new(player: Player) -> Chessboard {
      let mut fields = [[Field::new(); 8]; 8];

      // init player black
      fields[0][0].token = Token::Rook;
      fields[0][1].token = Token::Knight;
      fields[0][2].token = Token::Bishop;
      fields[0][3].token = Token::Queen;
      fields[0][4].token = Token::King;
      fields[0][5].token = Token::Bishop;
      fields[0][6].token = Token::Knight;
      fields[0][7].token = Token::Rook;
      for x in 0..8 {
        fields[0][x].player = Player::Black;
        fields[1][x].player = Player::Black;
        fields[1][x].token = Token::Pawn;
      }

      // init player white
      fields[7][0].token = Token::Rook;
      fields[7][1].token = Token::Knight;
      fields[7][2].token = Token::Bishop;
      fields[7][3].token = Token::Queen;
      fields[7][4].token = Token::King;
      fields[7][5].token = Token::Bishop;
      fields[7][6].token = Token::Knight;
      fields[7][7].token = Token::Rook;
      for x in 0..8 {
        fields[7][x].player = Player::White;
        fields[6][x].player = Player::White;
        fields[6][x].token = Token::Pawn;
      }

      Chessboard {
        fields: fields,
        perspective: player,
      }
    }
  }

  fn map_field(field: &Field) -> &'static str {
    match field.token {
      Token::None => " ",
      Token::Pawn => {
        if field.player == Player::Black {
          "\u{2659}"
        } else {
          "\u{265F}"
        }
      }
      Token::Rook => {
        if field.player == Player::Black {
          "\u{2656}"
        } else {
          "\u{265C}"
        }
      }
      Token::Knight => {
        if field.player == Player::Black {
          "\u{2658}"
        } else {
          "\u{265E}"
        }
      }
      Token::Bishop => {
        if field.player == Player::Black {
          "\u{2657}"
        } else {
          "\u{265D}"
        }
      }
      Token::King => {
        if field.player == Player::Black {
          "\u{2654}"
        } else {
          "\u{265A}"
        }
      }
      Token::Queen => {
        if field.player == Player::Black {
          "\u{2655}"
        } else {
          "\u{265B}"
        }
      }
    }
  }

  impl ::fmt::Display for Chessboard {
    fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
      write!(f, "  ")?;
      for i in 1..9 {
        write!(f, "  {} ", i)?;
      }

      write!(f, "\n  +")?;
      for _ in 0..8 {
        write!(f, "---+")?;
      }

      match self.perspective {
        Player::White => {
          for y in 0..8 {
            write!(f, "\n{} |", ((72 - y) as u8) as char)?;

            for x in 0..8 {
              write!(f, " {} |", map_field(&self.fields[y][x]))?;
            }

            write!(f, "\n  +")?;
            for _ in 0..8 {
              write!(f, "---+")?;
            }
          }
        }
        Player::Black => {
          for y in (0..8).rev() {
            write!(f, "\n{} |", ((72 - y) as u8) as char)?;

            for x in (0..8).rev() {
              write!(f, " {} |", map_field(&self.fields[y][x]))?;
            }

            write!(f, "\n  +")?;
            for _ in 0..8 {
              write!(f, "---+")?;
            }
          }
        }
        _ => {}
      }

      write!(f, "\n")
    }
  }
}

fn main() {
  let board = rustychess::Chessboard::new(rustychess::Player::White);
  println!("{}", board);
  println!("Player white: ");

  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();

  print!("{}[2J", 27 as char);
  println!("User typed: {}", buffer)
}
