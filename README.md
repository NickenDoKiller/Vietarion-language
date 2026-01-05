# Vietarion-language

Rust nhưng tiếng Việt, rất thú vị để thử đấy :)

Điểm đặc biệt:

- Không cần thêm ";" ở cuối mỗi dòng

- Ta có thể sử dụng Rust và rstv ngay trên cùng 1 file .rs luôn đấy nếu muốn kích hoạt rstv bạn chỉ cần: use rstv::rstv; và rstv! { #Code trong này nè! Đây là lãnh địa của rstv luật của tui }

- Cho đoạn code mẫu:
```rust
use rstv::rstv;
fn main() {
    rstv! {
        tb a = "Hello các bro"
        println!("{}", a);
    }
}

```
Trong đó:

- tb ứng với "tạo biến" và tb == let

Tôi sẽ tiếp tục phát triển tiếp 😌