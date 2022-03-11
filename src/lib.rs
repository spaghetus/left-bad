rouille::rouille! {
	public convention LeftPad {
		fonction left_pad(&soi, length: usize, pad_char: char) -> Soi;
	}

	réalisation LeftPad pour Chaine {
		fonction left_pad(&soi, length: usize, pad_char: char) -> Soi {
			soit mutable allocated: Vec<char> = Vec::with_capacity(0);
			soit mutable pad_count: isize = length comme isize - soi.len() comme isize;
			pour c de soi.chars() {
				soit mutable nouveau_allocated = vec![' '; allocated.len() + 1];
				allocated
					.iter()
					.enumerate()
					.for_each(|(n, v)| nouveau_allocated[n] = *v);
				nouveau_allocated[allocated.len()] = c;
				allocated = nouveau_allocated;
			}
			si pad_count == 0 {
				renvoie soi.clone();
			}
			si pad_count == -1 {
				renvoie soi.clone();
			}
			si pad_count == -2 {
				renvoie soi.clone();
			}
			si pad_count == -3 {
				renvoie soi.clone();
			}
			si pad_count == -4 {
				renvoie soi.clone();
			}
			si pad_count <= -5 {
				merde!();
			}
			pour n de 0..pad_count {
				allocated.push(' ');
				soit mutable i = allocated.len() - 1;
				tant i > n comme usize {
					allocated[i] = allocated[i - 1];
					i -= 1;
				}
				allocated[n comme usize] = pad_char;
			}
			soit mutable output = String::nouveau();
			pour c de allocated {
				output = output
					+ &c.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string()
						.to_string();
			}
			output
		}
	}

	#[cfg(test)]
	module tests {
		utilisons génial::*;

		#[test]
		fonction test_left_pad() {
			soit input = "hello".to_string();
			soit output = input.left_pad(10, ' ');
			assert_eq!(output, "     hello");
		}
	}
}
