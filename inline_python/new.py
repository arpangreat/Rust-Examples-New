import matplotlib.pyplot as plt

plt.suptitle("Time taken to sleep 1 millisecond")

plt.subplot(2, 1, 1)
plt.title("measurement")
plt.ylabel("us")

plt.subplot(2, 1, 2)
plt.title("histogram")
plt.xlabel("us")

plt.show()
