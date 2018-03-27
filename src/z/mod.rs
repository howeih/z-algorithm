pub struct Z<'a> {
    target: &'a str,
}

impl<'a> Z<'a> {
    pub fn new(target: &'a str) -> Z<'a> {
        Z { target: target }
    }

    fn find_match(&self, pattern_len: usize, z_array: &Vec<u32>) -> Vec<usize>{
        let mut pattern_match = Vec::<usize>::new();
        for zi in pattern_len+1.. z_array.len()-1 {
            if z_array[zi] as usize == pattern_len{
                pattern_match.push(zi - pattern_len-1 );
            }
        }
        pattern_match
    }

    pub fn search(&self, pattern: &str) -> Vec<usize> {
        let target: Vec<char> = self.target.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect();
        let mut search_vec = Vec::with_capacity(target.len() + pattern.len());
        let pattern_len = pattern.len();
        search_vec.extend(pattern);
        search_vec.push(0x00 as char);
        search_vec.extend(target);
        let z_array = self.find_z_array(&search_vec);
        self.find_match(pattern_len, &z_array)
    }

    fn find_z_array(&self, target: &Vec<char>) -> Vec<u32> {
        let t_len = target.len();
        let mut z_array = vec![0u32; t_len];
        z_array[0] = 1;
        for ti in 1..t_len {
            let mut t_start = 0;
            let mut z_value = 0;
            let mut t_itr = ti;
            while t_start < t_len && t_itr < t_len && target[t_start] == target[t_itr] {
                z_value += 1;
                t_start += 1;
                t_itr += 1;
            }
            z_array[ti] = z_value;
        }
        z_array
    }
}
