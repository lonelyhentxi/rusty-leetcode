#[macro_export]
macro_rules! lc_matrix {
  ( $( [ $( $x: expr ), * ] ),* ) => {
    {
      let mut all_vec = Vec::new();
      $(
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push( $x );
        )*
        all_vec.push(temp_vec);
      )*
      all_vec
    }
  };
}

#[macro_export]
macro_rules! lc_matrix_s {
  ( $( [ $( $x: expr ), * ] ),* ) => {
    {
      let mut all_vec = Vec::new();
      $(
        let mut temp_vec = Vec::<String>::new();
        $(
            temp_vec.push( String::from( $x ));
        )*
        all_vec.push(temp_vec);
      )*
      all_vec
    }
  };
}

#[macro_export]
macro_rules! lc_vec_s {
    () => { Vec::<String>::new() };
    ( $( $x:expr ),* ) => {
      {
          let mut temp_vec = Vec::<String>::new();
          $(
              temp_vec.push( String::from( $x ) );
          )*
          temp_vec
      }
  };
}

#[cfg(test)]
mod test {
    use crate::{lc_matrix, lc_matrix_s, lc_vec_s };

    #[test]
    fn test_matrix_macro_1() {
        let m1 = lc_matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let m2 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_matrix_macro_2() {
        let m1 = lc_matrix_s![["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];
        let m2 = vec![vec!["1", "2", "3"], vec!["4", "5", "6"], vec!["7", "8", "9"]];
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_vec_macro_1() {
        let m1 = lc_vec_s!["1", "2", "3"];
        let m2 = vec![String::from("1"), String::from("2"), String::from("3")];
        assert_eq!(m1, m2);
    }
}
