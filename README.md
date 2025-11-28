# ft_linear_regression

[![Rust](https://img.shields.io/badge/Rust-1.91+-blue?logo=rust)](https://www.rust-lang.org/)  
[![Python](https://img.shields.io/badge/Python-3.x-yellow?logo=python)](https://www.python.org/)  
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

A **simple linear regression project** implemented in **Rust**, predicting car prices based on mileage.  
It uses **manual gradient descent** (no ML libraries) and includes **bonus features**: dataset plotting, regression line visualization, and RMSE calculation.


## How to Run

### 1️⃣ Train the Model

```bash
$ cargo run --bin train

Iteration 0: Loss = 20469431.1280
Iteration 500: Loss = 223696.6939
Iteration 1000: Loss = 222822.6603
...
Training completed.
Theta0 = 6331.83
Theta1 = -1106.019881
```

### 2️⃣ Predict a Car Price
```bash
$ cargo run --bin predict
Enter mileage: 50000
Estimated price: 7100.42
```
---

## 📈 Next Steps

- Experiment with **different learning rates** to optimize convergence.  
- Extend the model by including **additional features** such as car age or engine size.  
- Use the **visualization tools** provided to better understand the dataset and regression results.  
- Explore **advanced regression techniques** once comfortable with linear regression fundamentals.  

## ⚙️ Contributions

Contributions, improvements, and suggestions are welcome. Feel free to **fork the repository** and submit a pull request.  


