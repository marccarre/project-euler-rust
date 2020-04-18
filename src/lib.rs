extern crate itertools;
extern crate num_integer;

mod iterators;
mod numbers;

use iterators::Fibonacci;
use iterators::Integers;
use iterators::Primes;
use itertools::Itertools;
use num::integer;
use num_integer::Integer;
use numbers::is_palindrome;
use numbers::is_prime;
use numbers::num_divisors;
use std::cmp;
use std::env;
use std::fs;
use std::path::PathBuf;

/**
 * Problem 1: Multiples of 3 and 5
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we
 * get 3, 5, 6 and 9. The sum of these multiples is 23.
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */
pub fn problem_0001(n: u32) -> u32 {
    return (1..n).filter(|k| k % 3 == 0 || k % 5 == 0).sum();
}

/**
 * Problem 2: Even Fibonacci numbers
 * Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
 *     1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
 * By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
 */
pub fn problem_0002_imperative(n: u32) -> u32 {
    let (mut i, mut j) = (0, 1);
    let mut fibo = i + j;
    let mut even_sum = 0;
    while fibo < n {
        if fibo % 2 == 0 {
            even_sum += fibo;
        }
        i = j;
        j = fibo;
        fibo = i + j;
    }
    return even_sum;
}

pub fn problem_0002_functional(n: u32) -> u32 {
    Fibonacci::<u32>::new()
        .take_while(|&k| k < n)
        .filter(|&k| k.is_even())
        .sum()
}

/**
 * Problem 3: Largest prime factor
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 */
pub fn problem_0003(n: u64) -> u64 {
    let mut k = n;
    let mut largest = 1;

    let mut primes = Primes::new(sqrt(n));
    loop {
        match primes.next() {
            Some(prime) => {
                while k != 1 && k % prime == 0 {
                    k = k / prime;
                    largest = prime;
                }
                if k == 1 {
                    break;
                }
            }
            None => break,
        }
    }
    return cmp::max(largest, k);
}

fn sqrt(n: u64) -> usize {
    (n as f64).sqrt() as usize
}

/**
 * Problem 4: Largest palindrome product
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
 */
pub fn problem_0004(x: u32, y: u32) -> u32 {
    // The number n we're interested in typically has the following structure:
    // - for 2-digit numbers forming it:
    //       1000*a + 100*b + 10*b + a = 1001*a + 110*b
    //     = 11*(91*a + 10*b)
    //     = 11*(13*7*a + 5*2*b)
    // - for 3-digit numbers forming it:
    //       100000*a + 10000*b + 1000*c + 100*c + 10*b + a
    //     = 100001*a + 10010*b + 1100*c
    //     = 11*(9091*a + 910*b + 100*c)
    //     = 11*(9091*a + 13*7*5*2*b + 5*5*2*2*c)
    // hence n must be divisible by 11.
    // This helps reduce the search space, and the number of calls to is_palindrome.
    return (x - x / 10..x)
        .cartesian_product(y - y / 10..y)
        .map(|(a, b)| a * b)
        .filter(|x| x % 11 == 0)
        .filter(|x| is_palindrome(x))
        .max()
        .unwrap();
}

/**
 * Problem 5: Smallest multiple
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 */
pub fn problem_0005(n: u32) -> u32 {
    (1..n).fold(1, |x, y| integer::lcm(x, y))
}

/**
 * Problem 6: Sum square difference
 * The sum of the squares of the first ten natural numbers is: 1**2 + 2**2 + ... + 10**2 = 385.
 * The square of the sum of the first ten natural numbers is: (1+2+...+10)**2 = 552 = 3025.
 * Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
 * Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 */
pub fn problem_0006(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2) - (1..=n).map(|x| x.pow(2)).sum::<u32>()
}

/**
 * Problem 7: 10001st prime
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 * What is the 10 001st prime number?
 */
pub fn problem_0007(n: usize) -> u32 {
    Integers::new().filter(|&k| is_prime(k)).nth(n - 1).unwrap()
}

/**
 * Problem 8: Largest product in a series
 * The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
 *
 *   73167176531330624919225119674426574742355349194934
 *   96983520312774506326239578318016984801869478851843
 *   85861560789112949495459501737958331952853208805511
 *   12540698747158523863050715693290963295227443043557
 *   66896648950445244523161731856403098711121722383113
 *   62229893423380308135336276614282806444486645238749
 *   30358907296290491560440772390713810515859307960866
 *   70172427121883998797908792274921901699720888093776
 *   65727333001053367881220235421809751254540594752243
 *   52584907711670556013604839586446706324415722155397
 *   53697817977846174064955149290862569321978468622482
 *   83972241375657056057490261407972968652414535100474
 *   82166370484403199890008895243450658541227588666881
 *   16427171479924442928230863465674813919123162824586
 *   17866458359124566529476545682848912883142607690042
 *   24219022671055626321111109370544217506941658960408
 *   07198403850962455444362981230987879927244284909188
 *   84580156166097919133875499200524063689912560717606
 *   05886116467109405077541002256983155200055935729725
 *   71636269561882670428252483600823257530420752963450
 *
 * Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
 */
pub fn problem_0008(window_size: usize) -> u64 {
    let digits = vec![
        7, 3, 1, 6, 7, 1, 7, 6, 5, 3, 1, 3, 3, 0, 6, 2, 4, 9, 1, 9, 2, 2, 5, 1, 1, 9, 6, 7, 4, 4,
        2, 6, 5, 7, 4, 7, 4, 2, 3, 5, 5, 3, 4, 9, 1, 9, 4, 9, 3, 4, 9, 6, 9, 8, 3, 5, 2, 0, 3, 1,
        2, 7, 7, 4, 5, 0, 6, 3, 2, 6, 2, 3, 9, 5, 7, 8, 3, 1, 8, 0, 1, 6, 9, 8, 4, 8, 0, 1, 8, 6,
        9, 4, 7, 8, 8, 5, 1, 8, 4, 3, 8, 5, 8, 6, 1, 5, 6, 0, 7, 8, 9, 1, 1, 2, 9, 4, 9, 4, 9, 5,
        4, 5, 9, 5, 0, 1, 7, 3, 7, 9, 5, 8, 3, 3, 1, 9, 5, 2, 8, 5, 3, 2, 0, 8, 8, 0, 5, 5, 1, 1,
        1, 2, 5, 4, 0, 6, 9, 8, 7, 4, 7, 1, 5, 8, 5, 2, 3, 8, 6, 3, 0, 5, 0, 7, 1, 5, 6, 9, 3, 2,
        9, 0, 9, 6, 3, 2, 9, 5, 2, 2, 7, 4, 4, 3, 0, 4, 3, 5, 5, 7, 6, 6, 8, 9, 6, 6, 4, 8, 9, 5,
        0, 4, 4, 5, 2, 4, 4, 5, 2, 3, 1, 6, 1, 7, 3, 1, 8, 5, 6, 4, 0, 3, 0, 9, 8, 7, 1, 1, 1, 2,
        1, 7, 2, 2, 3, 8, 3, 1, 1, 3, 6, 2, 2, 2, 9, 8, 9, 3, 4, 2, 3, 3, 8, 0, 3, 0, 8, 1, 3, 5,
        3, 3, 6, 2, 7, 6, 6, 1, 4, 2, 8, 2, 8, 0, 6, 4, 4, 4, 4, 8, 6, 6, 4, 5, 2, 3, 8, 7, 4, 9,
        3, 0, 3, 5, 8, 9, 0, 7, 2, 9, 6, 2, 9, 0, 4, 9, 1, 5, 6, 0, 4, 4, 0, 7, 7, 2, 3, 9, 0, 7,
        1, 3, 8, 1, 0, 5, 1, 5, 8, 5, 9, 3, 0, 7, 9, 6, 0, 8, 6, 6, 7, 0, 1, 7, 2, 4, 2, 7, 1, 2,
        1, 8, 8, 3, 9, 9, 8, 7, 9, 7, 9, 0, 8, 7, 9, 2, 2, 7, 4, 9, 2, 1, 9, 0, 1, 6, 9, 9, 7, 2,
        0, 8, 8, 8, 0, 9, 3, 7, 7, 6, 6, 5, 7, 2, 7, 3, 3, 3, 0, 0, 1, 0, 5, 3, 3, 6, 7, 8, 8, 1,
        2, 2, 0, 2, 3, 5, 4, 2, 1, 8, 0, 9, 7, 5, 1, 2, 5, 4, 5, 4, 0, 5, 9, 4, 7, 5, 2, 2, 4, 3,
        5, 2, 5, 8, 4, 9, 0, 7, 7, 1, 1, 6, 7, 0, 5, 5, 6, 0, 1, 3, 6, 0, 4, 8, 3, 9, 5, 8, 6, 4,
        4, 6, 7, 0, 6, 3, 2, 4, 4, 1, 5, 7, 2, 2, 1, 5, 5, 3, 9, 7, 5, 3, 6, 9, 7, 8, 1, 7, 9, 7,
        7, 8, 4, 6, 1, 7, 4, 0, 6, 4, 9, 5, 5, 1, 4, 9, 2, 9, 0, 8, 6, 2, 5, 6, 9, 3, 2, 1, 9, 7,
        8, 4, 6, 8, 6, 2, 2, 4, 8, 2, 8, 3, 9, 7, 2, 2, 4, 1, 3, 7, 5, 6, 5, 7, 0, 5, 6, 0, 5, 7,
        4, 9, 0, 2, 6, 1, 4, 0, 7, 9, 7, 2, 9, 6, 8, 6, 5, 2, 4, 1, 4, 5, 3, 5, 1, 0, 0, 4, 7, 4,
        8, 2, 1, 6, 6, 3, 7, 0, 4, 8, 4, 4, 0, 3, 1, 9, 9, 8, 9, 0, 0, 0, 8, 8, 9, 5, 2, 4, 3, 4,
        5, 0, 6, 5, 8, 5, 4, 1, 2, 2, 7, 5, 8, 8, 6, 6, 6, 8, 8, 1, 1, 6, 4, 2, 7, 1, 7, 1, 4, 7,
        9, 9, 2, 4, 4, 4, 2, 9, 2, 8, 2, 3, 0, 8, 6, 3, 4, 6, 5, 6, 7, 4, 8, 1, 3, 9, 1, 9, 1, 2,
        3, 1, 6, 2, 8, 2, 4, 5, 8, 6, 1, 7, 8, 6, 6, 4, 5, 8, 3, 5, 9, 1, 2, 4, 5, 6, 6, 5, 2, 9,
        4, 7, 6, 5, 4, 5, 6, 8, 2, 8, 4, 8, 9, 1, 2, 8, 8, 3, 1, 4, 2, 6, 0, 7, 6, 9, 0, 0, 4, 2,
        2, 4, 2, 1, 9, 0, 2, 2, 6, 7, 1, 0, 5, 5, 6, 2, 6, 3, 2, 1, 1, 1, 1, 1, 0, 9, 3, 7, 0, 5,
        4, 4, 2, 1, 7, 5, 0, 6, 9, 4, 1, 6, 5, 8, 9, 6, 0, 4, 0, 8, 0, 7, 1, 9, 8, 4, 0, 3, 8, 5,
        0, 9, 6, 2, 4, 5, 5, 4, 4, 4, 3, 6, 2, 9, 8, 1, 2, 3, 0, 9, 8, 7, 8, 7, 9, 9, 2, 7, 2, 4,
        4, 2, 8, 4, 9, 0, 9, 1, 8, 8, 8, 4, 5, 8, 0, 1, 5, 6, 1, 6, 6, 0, 9, 7, 9, 1, 9, 1, 3, 3,
        8, 7, 5, 4, 9, 9, 2, 0, 0, 5, 2, 4, 0, 6, 3, 6, 8, 9, 9, 1, 2, 5, 6, 0, 7, 1, 7, 6, 0, 6,
        0, 5, 8, 8, 6, 1, 1, 6, 4, 6, 7, 1, 0, 9, 4, 0, 5, 0, 7, 7, 5, 4, 1, 0, 0, 2, 2, 5, 6, 9,
        8, 3, 1, 5, 5, 2, 0, 0, 0, 5, 5, 9, 3, 5, 7, 2, 9, 7, 2, 5, 7, 1, 6, 3, 6, 2, 6, 9, 5, 6,
        1, 8, 8, 2, 6, 7, 0, 4, 2, 8, 2, 5, 2, 4, 8, 3, 6, 0, 0, 8, 2, 3, 2, 5, 7, 5, 3, 0, 4, 2,
        0, 7, 5, 2, 9, 6, 3, 4, 5, 0,
    ];
    digits
        .windows(window_size)
        .map(|window| window.iter().fold(1u64, |product, &digit| product * digit))
        .max()
        .unwrap()
}

/**
 * Problem 9: Special Pythagorean triplet
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a**2 + b**2 = c**2
 * For example, 3**2 + 4**2 = 9 + 16 = 25 = 5**2.
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * Find the product abc.
 */
pub fn problem_0009(target_sum: u32) -> u32 {
    for i in 1..target_sum / 2 {
        for j in i + 1..target_sum / 2 {
            let k = target_sum - i - j;
            if j < k && i * i + j * j == k * k {
                return i * j * k;
            }
        }
    }
    return 0;
}

/**
 * Problem 10: Summation of primes
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 */
pub fn problem_0010(bound: usize) -> u64 {
    Primes::new(bound).sum()
}

/**
 * Problem 11: Largest product in a grid
 * In the 20×20 grid below, four numbers along a diagonal line have been marked with brackets.
 *
 * 08 02 22 97 38 15 00 40  00  75  04  05  07 78 52 12 50 77 91 08
 * 49 49 99 40 17 81 18 57  60  87  17  40  98 43 69 48 04 56 62 00
 * 81 49 31 73 55 79 14 29  93  71  40  67  53 88 30 03 49 13 36 65
 * 52 70 95 23 04 60 11 42  69  24  68  56  01 32 56 71 37 02 36 91
 * 22 31 16 71 51 67 63 89  41  92  36  54  22 40 40 28 66 33 13 80
 * 24 47 32 60 99 03 45 02  44  75  33  53  78 36 84 20 35 17 12 50
 * 32 98 81 28 64 23 67 10 [26] 38  40  67  59 54 70 66 18 38 64 70
 * 67 26 20 68 02 62 12 20  95 [63] 94  39  63 08 40 91 66 49 94 21
 * 24 55 58 05 66 73 99 26  97  17 [78] 78  96 83 14 88 34 89 63 72
 * 21 36 23 09 75 00 76 44  20  45  35 [14] 00 61 33 97 34 31 33 95
 * 78 17 53 28 22 75 31 67  15  94  03  80  04 62 16 14 09 53 56 92
 * 16 39 05 42 96 35 31 47  55  58  88  24  00 17 54 24 36 29 85 57
 * 86 56 00 48 35 71 89 07  05  44  44  37  44 60 21 58 51 54 17 58
 * 19 80 81 68 05 94 47 69  28  73  92  13  86 52 17 77 04 89 55 40
 * 04 52 08 83 97 35 99 16  07  97  57  32  16 26 26 79 33 27 98 66
 * 88 36 68 87 57 62 20 72  03  46  33  67  46 55 12 32 63 93 53 69
 * 04 42 16 73 38 25 39 11  24  94  72  18  08 46 29 32 40 62 76 36
 * 20 69 36 41 72 30 23 88  34  62  99  69  82 67 59 85 74 04 36 16
 * 20 73 35 29 78 31 90 01  74  31  49  71  48 86 81 16 23 57 05 54
 * 01 70 54 71 83 51 54 69  16  92  33  48  61 43 52 01 89 19 67 48
 *
 * The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
 * What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
*/
pub fn problem_0011(window_size: usize) -> u32 {
    let nums: Vec<u32> = vec![
        8, 2, 22, 97, 38, 15, 0, 40, 0, 75, 4, 5, 7, 78, 52, 12, 50, 77, 91, 8, 49, 49, 99, 40, 17,
        81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0, 81, 49, 31, 73, 55, 79, 14, 29,
        93, 71, 40, 67, 53, 88, 30, 3, 49, 13, 36, 65, 52, 70, 95, 23, 4, 60, 11, 42, 69, 24, 68,
        56, 1, 32, 56, 71, 37, 2, 36, 91, 22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40,
        40, 28, 66, 33, 13, 80, 24, 47, 32, 60, 99, 3, 45, 2, 44, 75, 33, 53, 78, 36, 84, 20, 35,
        17, 12, 50, 32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        67, 26, 20, 68, 2, 62, 12, 20, 95, 63, 94, 39, 63, 8, 40, 91, 66, 49, 94, 21, 24, 55, 58,
        5, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72, 21, 36, 23, 9, 75, 0,
        76, 44, 20, 45, 35, 14, 0, 61, 33, 97, 34, 31, 33, 95, 78, 17, 53, 28, 22, 75, 31, 67, 15,
        94, 3, 80, 4, 62, 16, 14, 9, 53, 56, 92, 16, 39, 5, 42, 96, 35, 31, 47, 55, 58, 88, 24, 0,
        17, 54, 24, 36, 29, 85, 57, 86, 56, 0, 48, 35, 71, 89, 7, 5, 44, 44, 37, 44, 60, 21, 58,
        51, 54, 17, 58, 19, 80, 81, 68, 5, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 4, 89, 55,
        40, 4, 52, 8, 83, 97, 35, 99, 16, 7, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66, 88, 36,
        68, 87, 57, 62, 20, 72, 3, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69, 4, 42, 16, 73, 38,
        25, 39, 11, 24, 94, 72, 18, 8, 46, 29, 32, 40, 62, 76, 36, 20, 69, 36, 41, 72, 30, 23, 88,
        34, 62, 99, 69, 82, 67, 59, 85, 74, 4, 36, 16, 20, 73, 35, 29, 78, 31, 90, 1, 74, 31, 49,
        71, 48, 86, 81, 16, 23, 57, 5, 54, 1, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43,
        52, 1, 89, 19, 67, 48,
    ];
    let matrix: Vec<&[u32]> = nums.chunks(20).collect();

    let n = matrix.len();
    let m = matrix[0].len();

    let mut cells_ranges: Vec<Vec<(usize, usize)>> = vec![];
    cells_ranges.extend((0..n).map(|i| (0..m).map(|j| (i, j)).collect())); // rows
    cells_ranges.extend((0..m).map(|j| (0..n).map(|i| (i, j)).collect())); // columns

    // top-left to bottom-right diagonals:
    cells_ranges.extend((0..n).map(|i| (0..cmp::min(n - i, m)).map(|k| (i + k, k)).collect()));
    cells_ranges.extend((1..m).map(|j| (0..cmp::min(n, m - j)).map(|k| (k, j + k)).collect()));
    // bottom-left to top-right diagonals:
    cells_ranges.extend((1..n).map(|i| (0..cmp::min(i + 1, m)).map(|k| (i - k, k)).collect()));
    cells_ranges.extend((1..m).map(|j| (0..cmp::min(n, m - j)).map(|k| (k, j + k)).collect()));

    cells_ranges
        .iter()
        .filter(|range| range.len() >= window_size) // Remove ranges which are too small.
        .map(|range| {
            range
                .windows(window_size)
                .map(|cells| cells.iter().map(|&(i, j)| matrix[i][j]).product())
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

/**
 * Problem 12: Highly divisible triangular number
 * The sequence of triangle numbers is generated by adding the natural numbers.
 * So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.
 * The first ten terms would be: 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
 * Let us list the factors of the first seven triangle numbers:
 *
 *  1: 1
 *  3: 1,3
 *  6: 1,2,3,6
 * 10: 1,2,5,10
 * 15: 1,3,5,15
 * 21: 1,3,7,21
 * 28: 1,2,4,7,14,28
 *
 * We can see that 28 is the first triangle number to have over five divisors.
 * What is the value of the first triangle number to have over five hundred divisors?
*/
pub fn problem_0012(max_divisors: u32) -> u32 {
    let triangles = (1..).scan(0, |triangle, n| {
        *triangle += n;
        Some(*triangle)
    });
    triangles
        .skip_while(|&triangle| num_divisors(triangle) <= max_divisors)
        .next()
        .unwrap()
}

/**
 * Problem 13: Large sum
 * Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
 */
pub fn problem_0013(num_digits: usize) -> u64 {
    let filepath = project_dir().join("tests/problem_0013.txt");
    let file = fs::read_to_string(filepath).expect("Failed to read tests/problem_0013.txt");
    let numbers: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.to_string().chars().collect())
        .collect();

    const NUM_DIGITS: usize = 50;
    const RADIX: u32 = 10;
    let mut sum = Vec::new();
    let mut carry: u32 = 0;
    for i in 0..NUM_DIGITS {
        let digits_sum = numbers
            .iter()
            .map(|num| num[NUM_DIGITS - i - 1])
            .map(|c| c.to_digit(RADIX).unwrap())
            .sum::<u32>()
            + carry;
        carry = digits_sum / 10;
        let digit = digits_sum % 10;
        sum.push(digit.to_string());
    }
    sum.push(carry.to_string());
    return sum
        .join("")
        .chars()
        .rev()
        .take(num_digits)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
}

fn project_dir() -> PathBuf {
    // Test binary typically lives under: target/debug/deps/
    let exe = env::current_exe().expect("Error returning this executable's filepath");
    let deps = exe.parent().expect("Failed to access deps directory");
    let debug = deps.parent().expect("Failed to access debug directory");
    let target = debug.parent().expect("Failed to access target directory");
    let project = target.parent().expect("Failed to access project directory");
    project.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_problem_0001() {
        assert_eq!(problem_0001(1000), 233168);
    }

    #[test]
    fn test_problem_0002() {
        assert_eq!(problem_0002_imperative(4_000_000), 4613732);
        assert_eq!(problem_0002_functional(4_000_000), 4613732);
    }

    #[test]
    fn test_problem_0003() {
        assert_eq!(problem_0003(13_195), 29);
        assert_eq!(problem_0003(600_851_475_143), 6857);
    }

    #[test]
    fn test_problem_0004() {
        assert_eq!(problem_0004(100, 100), 9009);
        assert_eq!(problem_0004(1000, 1000), 906609);
    }

    #[test]
    fn test_problem_0005() {
        assert_eq!(problem_0005(10), 2520);
        assert_eq!(problem_0005(20), 232792560);
    }

    #[test]
    fn test_problem_0006() {
        assert_eq!(problem_0006(10), 2640);
        assert_eq!(problem_0006(100), 25164150);
    }

    #[test]
    fn test_problem_0007() {
        assert_eq!(problem_0007(6), 13);
        assert_eq!(problem_0007(10001), 104743);
    }

    #[test]
    fn test_problem_0008() {
        assert_eq!(problem_0008(4), 5832);
        assert_eq!(problem_0008(13), 23514624000);
    }

    #[test]
    fn test_problem_0009() {
        assert_eq!(problem_0009(12), 60);
        assert_eq!(problem_0009(1000), 31875000);
    }

    #[test]
    fn test_problem_0010() {
        assert_eq!(problem_0010(10), 17);
        assert_eq!(problem_0010(2_000_000), 142913828922);
    }

    #[test]
    fn test_problem_0011() {
        assert_eq!(problem_0011(4), 70600674);
    }

    #[test]
    fn test_problem_0012() {
        assert_eq!(problem_0012(5), 28);
        assert_eq!(problem_0012(500), 76576500);
    }

    #[test]
    fn test_problem_0013() {
        assert_eq!(problem_0013(10), 5537376230);
    }
}
