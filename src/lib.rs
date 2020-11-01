extern crate suffix;
use suffix::SuffixTable;
/// Compute the longest common substring among a slice of strings
///
/// # Example
///
/// ```
/// let lcs = longest_common_substring(&[
///     "ZYABCAGB",
///     "BCAGDTZYY",
///     "DACAGZZYSC",
///     "CAGYZYSAU",
///     "CAZYUCAGF",
/// ]);
/// assert_eq!(lcs, "CAG");
/// ```
pub fn longest_common_substring<'a>(strs: &[&'a str]) -> &'a str {
    let number_of_strings = strs.len();
    let mut boundaries = Vec::new();
    let mut concatenated = String::new();
    for s in strs.iter() {
        concatenated.push_str(*s);
        boundaries.push(concatenated.len());
    }
    let sa = SuffixTable::new(&concatenated);
    let pos = sa.table();
    let lcp = sa.lcp_lens();
    let mut lcs_len = 0u32;
    let mut lcs_pos: u32 = 0;

    'outer: for (win_p, win_l) in pos
        .windows(number_of_strings)
        .zip(lcp.windows(number_of_strings))
    {
        // examine if each window contains substrings coming from different original strings
        // use a vector of booleans to track whether a substring has been included.
        // upon duplication, abort and continue scanning the next frame
        let mut included = vec![false; number_of_strings];
        for &p in win_p {
            let n = string_number(p, &boundaries);
            if included[n] {
                continue 'outer;
            } else {
                included[n] = true;
            }
        }
        // this window contains one and only one suffixes from each original strings
        // calculate the LCS within this window
        let m = win_l.iter().skip(1).min().unwrap(); // win_l always has length 3
        let this_cs_len = *m;
        let this_cs_pos = win_p[0];
        // let this_cs = concatenated
        //     .chars()
        //     .skip(this_cs_pos as usize)
        //     .take(this_cs_len as usize)
        //     .collect::<String>();

        if *m > lcs_len {
            lcs_len = this_cs_len;
            lcs_pos = this_cs_pos;
        }
    }

    let lcs_len = lcs_len as usize;
    let lcs_pos = lcs_pos as usize;

    std::str::from_utf8(&strs[0].as_bytes()[lcs_pos..lcs_pos + lcs_len]).unwrap()
}

/// compute a vector of common substring among a slice of input substrings. The output is sorted lexicographically.
pub fn common_substrings<'a>(strs: &[&'a str]) -> Vec<&'a str> {
    let number_of_strings = strs.len();
    let mut boundaries = Vec::new();
    let mut concatenated = String::new();
    for s in strs.iter() {
        concatenated.push_str(*s);
        boundaries.push(concatenated.len());
    }
    let sa = SuffixTable::new(&concatenated);
    let pos = sa.table();
    let lcp = sa.lcp_lens();

    let mut css = Vec::new();

    'outer: for (win_p, win_l) in pos
        .windows(number_of_strings)
        .zip(lcp.windows(number_of_strings))
    {
        // examine if each window contains substrings coming from different original strings
        // use a vector of booleans to track whether a substring has been included.
        // upon duplication, abort and continue scanning the next frame
        let mut included = vec![false; number_of_strings];
        for &p in win_p {
            let n = string_number(p, &boundaries);
            if included[n] {
                continue 'outer;
            } else {
                included[n] = true;
            }
        }
        // this window contains one and only one suffixes from each original strings
        // calculate the LCS within this window
        let m = win_l.iter().skip(1).min().unwrap(); // win_l always has length 3
        let this_cs_len = *m as usize;
        let this_cs_pos = win_p[0] as usize;
        let this_cs =
            std::str::from_utf8(&strs[0].as_bytes()[this_cs_pos..this_cs_pos + this_cs_len])
                .unwrap();
        css.push(this_cs);
    }

    css
}

fn string_number(position: u32, boundaries: &[usize]) -> usize {
    match boundaries.binary_search(&(position as usize)) {
        Ok(idx) => idx + 1,
        Err(idx) => idx,
    }
}
