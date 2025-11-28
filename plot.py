import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import csv
import json
import numpy as np
import math

x = []
y = []
with open("data.csv") as f:
    reader = csv.DictReader(f)
    for row in reader:
        x.append(float(row["km"]))
        y.append(float(row["price"]))

with open("model.json") as f:
    model = json.load(f)
    theta0 = model["theta0"]
    theta1 = model["theta1"]
    mean = model.get("mean", 0)
    std = model.get("std", 1)

x_line = np.linspace(min(x), max(x), 100)
x_line_norm = [(xi - mean)/std for xi in x_line]
y_line = [theta0 + theta1 * xi for xi in x_line_norm]

plt.scatter(x, y, color='blue', label='Data points')
plt.plot(x_line, y_line, color='red', label='Regression line')
plt.xlabel('Mileage (km)')
plt.ylabel('Price')
plt.title('Car Price vs Mileage')
plt.legend()
plt.savefig("linear_regression_plot.png")
print("Plot saved as linear_regression_plot.png")

y_pred_dataset = [theta0 + theta1 * ((xi - mean)/std) for xi in x]
rmse = math.sqrt(sum((y_pred_dataset[i] - y[i])**2 for i in range(len(y))) / len(y))
print(f"RMSE: {rmse:.2f}")



# Blue dots → actual car prices

# Red line → predicted prices
