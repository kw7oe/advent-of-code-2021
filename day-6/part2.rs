fn main() {
    let mut intervals: [usize; 9] =
        include_str!("input.txt")
            .trim()
            .split(",")
            .fold([0; 9], |mut arr, n| {
                arr[n.parse::<usize>().unwrap()] += 1;
                arr
            });

    let days = 256;

    (1..=days).for_each(|day| {
        println!("day {}", day);
        let mut temp = 0;

        (0..=8).for_each(|n| {
            // Since, interval 0 give to interval 8 and interval 6,
            // which is not after each other.
            //
            // we store it in a temporary value first
            // so we can add to interval 8 and interval 6 later.
            if n == 0 {
                temp = intervals[n];
            }

            if n != 8 {
                // interval 1 be replaced from interval 2
                intervals[n] = intervals[n + 1];
            }

            // interval 6 will also receive from interval 0,
            // as we revive
            if n == 6 {
                intervals[n] += temp;
            }

            // interval 8 will receive from interval 0,
            // as we spawn new lattern.
            if n == 8 {
                intervals[n] = temp;
            }
        });
    });

    println!("Total: {}", intervals.iter().sum::<usize>());
}
