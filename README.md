# 🎄 Advent of Code 2021 🎄

I decided to stick with Rust this year and try to improve a bit on it, I basically haven't used it since last year's AoC, so you 
could say I'm a bit RUSTy with it 🥶.

I'm not going to shoot for the leaderboard, the goal is getting all the 50 ⭐ while learning and having fun.

Not only that, but I'll try my best to get every ⭐ within 24 hours of the puzzle release.

## ❄️ How to use ❄️
`cargo run -p day*` - Runs a specific day

`cargo test -p day*` - Tests a specific day

`cargo test` - Tests all

## 🥛 Results 🍪
| Day | Part 1 Time | Part 1 Rank | Part 1 Runtime[^1] | Part 2 Time | Part 2 Rank | Part 2 Runtime[^1] |
|:-:|-:|-:|-:|-:|-:|-:|
|  1 | 00:13:19 |  5740 |  19.5µs | 00:21:33 |  5187 |  20.7µs |
|  2 | 02:31:32 | 25070 |  88.2µs | 02:35:27 | 23492 |  76.4µs |
|  3 | 02:55:11 | 26333 |  47.2µs | 03:33:14 | 18066 | 345.3µs |
|  4 | 04:07:12 | 15365 | 172.3µs | 04:28:18 | 14328 | 313.7µs |
|  5 | 03:56:48 | 15828 |   3.4ms | 04:54:06 | 16137 |   1.7ms |
|  6 | 02:33:47 | 17752 |   5.6µs | 02:42:10 | 13212 |     5µs |
|  7 | 02:31:57 | 19790 |  59.8µs | 02:45:38 | 18822 |  26.5µs |
|  8 | 02:38:27 | 17947 |  89.4µs | 05:38:06 | 14742 | 972.8µs |
|  9 | 02:58:36 | 17499 | 321.9µs | 06:26:57 | 18828 | 506.1µs |
| 10 | 03:05:07 | 16485 |  78.9µs | 03:36:15 | 16034 |  99.1µs |
| 11 | 03:46:34 | 11792 | 246.8µs | 03:51:13 | 11638 | 791.1µs |
| 12 | 03:07:44 |  9230 |  10.4ms | 03:15:37 |  8001 | 305.5ms |

## 🎅 Have a Wonderful Holiday Season, Everyone! 🎅 

![koch flakes](https://raw.githubusercontent.com/fratorgano/advent_of_code_2020/main/snow.gif)


[^1]: `cargo run -p day* --release`, does not include the reading of the input file but includes parsing.
