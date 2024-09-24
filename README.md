# Overview
 Spear is a high-performance legal chess move generation library written in Rust, designed to enhance the capabilities of my chess engine, Javelin. This library is optimized for speed and efficiency, making it an ideal choice for developers looking to integrate powerful move generation into their chess applications. 
# Perft Results
 Following tests are conducted on Ryzen 9 7950x.

| Position | Depth | Nodes | Time | Nps |
| :-: | :-: | :-: | :-: | :-: |
| Start Position | 7 | 3.195b | 6.09s | 524.60m |
| Kiwipete | 6 | 8.031b | 11.44s | 701.76m |
| Position 3 | 8 | 3.009b | 6.54s | 459.65m |
| Position 4 | 6 | 706.04m | 0.99s | 711.02m |
| Position 5 | 6 | 3.048b | 4.90s | 620.94m |
| Position 6 | 6 | 6.923b | 9.42s | 734.61m |

| Position | Depth | Nodes | Time | Nps |
| :-: | :-: | :-: | :-: | :-: |
| Start Position | 6 | 119.06m | 0.70s | 169.78m |
| Kiwipete | 5 | 193.69m | 1.08s | 178.51m |
| Position 3 | 7 | 178.63m | 1.11s | 160.38m |
| Position 4 | 5 | 15.83m | 0.09s | 172.10m |
| Position 5 | 5 | 89.94m | 0.49s | 181.05m |
| Position 6 | 5 | 164.07m | 0.87s | 186.60m |
