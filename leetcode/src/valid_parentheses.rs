pub fn is_valid(s: String) -> bool {
    let mut st = Vec::new();
   for i in s.chars() {
       match i {
           '{' => st.push('}'),
           '(' => st.push(')'),
           '[' => st.push(']'),
           '}'|')'|']' if Some(i) != st.pop() => return false,
           _ => (),
       }
   }
   println!("   valid_parentheses.rs: \n {}", st.is_empty());
   return st.is_empty();
   }