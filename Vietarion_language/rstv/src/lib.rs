use proc_macro::TokenStream;
use proc_macro2::{TokenTree, TokenStream as TokenStream2, Punct, Spacing};

/// Entry point của macro rstv! { ... }
/// Nó nhận vào một đống Token (mã nguồn mày viết) và trả về code Rust chuẩn.
#[proc_macro]
pub fn rstv(input: TokenStream) -> TokenStream {
    // 1. Chuyển từ TokenStream (của compiler) sang TokenStream2 (để xử lý linh hoạt hơn)
    let input2 = TokenStream2::from(input);
    
    // 2. Dùng peekable để có thể "ngó trước" token tiếp theo mà không cần lấy nó ra ngay
    let mut tokens = input2.into_iter().peekable();
    
    // 3. Nơi chứa code Rust "đã qua chế biến"
    let mut output = TokenStream2::new();

    // 4. Bắt đầu duyệt từng Token trong lãnh địa
    while let Some(token) = tokens.next() {
        // Sao lưu token hiện tại để dùng cho việc kiểm tra chèn dấu chấm phẩy ở dưới
        // (Tránh lỗi Borrow Checker vì token gốc có thể bị Move vào trong match)
        let current_token_cloned = token.clone();

        match token {
            // TRƯỜNG HỢP A: Nếu gặp từ định danh (Identifier) là "tb"
            TokenTree::Ident(ref ident) if ident.to_string() == "tb" => {
                // Tạo một token mới là "let" nhưng giữ nguyên vị trí (span) để báo lỗi chuẩn
                let let_ident = syn::Ident::new("let", ident.span());
                output.extend(std::iter::once(TokenTree::Ident(let_ident)));
            }
            
            // TRƯỜNG HỢP B: Nếu gặp các cặp ngoặc {}, (), []
            TokenTree::Group(ref group) => {
                // ĐỆ QUY: Chui vào bên trong cặp ngoặc để xử lý tiếp (ví dụ tb trong if { ... })
                let inner_stream = rstv(group.stream().into());
                
                // Sau khi xử lý xong bên trong, đóng gói nó lại vào cặp ngoặc cũ
                let mut new_group = proc_macro2::Group::new(group.delimiter(), inner_stream.into());
                new_group.set_span(group.span()); // Giữ nguyên vị trí để debug
                output.extend(std::iter::once(TokenTree::Group(new_group)));
            }

            // TRƯỜNG HỢP C: Các token khác (số, toán tử, tên biến...) giữ nguyên
            _ => output.extend(std::iter::once(token)),
        }

        // 🔥 LOGIC CHÈN DẤU CHẤM PHẨY TỰ ĐỘNG (KIM CHỈ NAM CỦA LÃNH ĐỊA)
        // Nếu vẫn còn token tiếp theo trong danh sách
        if let Some(next) = tokens.peek() {
            // Kiểm tra xem token hiện tại và token tiếp theo có tạo thành điểm ngắt câu không
            if is_start_of_new_stmt(&current_token_cloned, next) {
                // Tự động chèn thêm dấu ';' vào output cho Rust vừa lòng
                output.extend(std::iter::once(TokenTree::Punct(Punct::new(';', Spacing::Alone))));
            }
        }
    }

    // Trả lại đống code đã "mông má" cho trình biên dịch Rust
    output.into()
}

/// Hàm bổ trợ: Quyết định xem có nên chèn dấu chấm phẩy hay không
fn is_start_of_new_stmt(current: &TokenTree, next: &TokenTree) -> bool {
    let next_str = next.to_string();

    // KIỂM TRA 1: Token vừa rồi có phải là thứ kết thúc một biểu thức không?
    let current_is_end = match current {
        TokenTree::Literal(_) => true, // Ví dụ: 10, "hello" (vừa gán xong giá trị)
        TokenTree::Ident(_)   => true, // Ví dụ: ten_bien (vừa dùng biến xong)
        // Nếu là dấu đóng ngoặc đơn ')' (ví dụ: xong một hàm println!(...))
        TokenTree::Group(g) => g.delimiter() == proc_macro2::Delimiter::Parenthesis,
        _ => false,
    };

    // KIỂM TRA 2: Token tiếp theo có phải là từ khóa bắt đầu một câu lệnh mới không?
    // Danh sách các "tín hiệu" bắt đầu câu lệnh trong lãnh địa của mày
    let next_is_start = [
        "tb",      // Khai báo biến mới
        "println", // Gọi hàm in
        "if",      // Cấu trúc rẽ nhánh
        "let",     // Đề phòng mày vẫn dùng let
        "loop",    // Vòng lặp
        "match",   // Khớp mẫu
        "return"   // Trả về giá trị
    ].contains(&next_str.as_str());

    // Nếu cả 2 đều đúng -> Chèn dấu chấm phẩy ngay và luôn!
    current_is_end && next_is_start
}