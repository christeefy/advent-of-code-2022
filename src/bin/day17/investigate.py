import pandas as pd

df = pd.read_csv("data.csv").astype("Int32")
breakpoint()
for i in range(len(df - 1)):
    res = pd.concat([df['1'], df['1'].shift(i)], axis=1).dropna()
    if (res.iloc[:, 0] == res.iloc[:, 1]).all():
        print(i)

# import itertools

# # of course this is a fake one just to offer an example
# def source():
#     return itertools.cycle((1, 0, 1, 4, 8, 2, 1, 3, 3, 2))

# import matplotlib.pyplot as plt
# import numpy as np
# import scipy as sp

# # Generate some test data, i.e. our "observations" of the signal
# N = len(df)
# # vals = iter(df['1'])
# X = df['1']

# # Compute the FFT
# W    = np.fft.fft(X)
# freq = np.fft.fftfreq(N,1)

# # Look for the longest signal that is "loud"
# threshold = 10**2
# idx = np.where(abs(W)>threshold)[0][-1]

# max_f = abs(freq[idx])
# print("Period estimate: ", 1/max_f)


# plt.subplot(211)
# plt.scatter([max_f,], [np.abs(W[idx]),], s=100,color='r')
# plt.plot(freq[:N/2], abs(W[:N/2]))
# plt.xlabel(r"$f$")

# plt.subplot(212)
# plt.plot(1.0/freq[:N/2], abs(W[:N/2]))
# plt.scatter([1/max_f,], [np.abs(W[idx]),], s=100,color='r')
# plt.xlabel(r"$1/f$")
# plt.xlim(0,20)