
---

# **Progrex** 🚀  
*A lightweight and customizable CLI progress bar for Rust applications.*  

[![Crates.io](https://img.shields.io/crates/v/progrex)](https://crates.io/crates/progrex)  
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)  

## **📌 Features**  
✅ Simple and easy-to-use API  
✅ Customizable progress bar styles  
✅ Real-time ETA tracking  
✅ Ideal for CLI applications  

---

## **📦 Installation**  
Add `progrex` to your `Cargo.toml`:  

```toml
[dependencies]
progrex = "0.1.0"
```

Then run:  
```sh
cargo build
```

---

## **🚀 Usage**  

### **Basic Example**  
```rust
use progrex::ProgressBar;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut bar = ProgressBar::new(100);

    for i in 0..=100 {
        bar.set_progress(i);
        sleep(Duration::from_millis(50));
    }

    bar.finish();
}
```

### **Output:**  
```
[███████████████               ] 50.00% | ETA: 2.5s
```

---

## **🔧 Customization**  
You can modify the bar length, display format, and refresh rate (coming soon).  

---

## **📜 License**  
Licensed under the **MIT License**. See [LICENSE](LICENSE) for details.  

---

## **🤝 Contributing**  
Contributions are welcome! Feel free to open issues or submit pull requests.  

---

Would you like to add badges for GitHub actions (build status) or more customization options? 😊