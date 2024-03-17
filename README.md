# A library for the Ramer-Douglas-Peucker-algorithm for simplifying sequences of points and saving them

See Wikipedia for the [https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm](RDP algorithm). 

This library shall provide ways to read curve-sequences of n-dimensional points and save only "important" points in order to save space. Typically RDP can reduce the size of a file to a small fraction without loosing much of the quality. RDP is a lossy algorithm.