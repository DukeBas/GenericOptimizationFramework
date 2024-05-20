
import random

# Define the number of points
num_points = 10000

# Open a file in write mode
with open("tsp_hardest_case.in", "w") as file:
    # Write the number of points as the first line
    file.write(f"{num_points}\n")
    
    # Generate and write each point with random coordinates
    for _ in range(num_points):
        x = random.randint(0, 10000)
        y = random.randint(0, 10000)
        file.write(f"{x} {y}\n")
