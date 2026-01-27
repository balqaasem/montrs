fn main() {
    // A comment outside
    view! {
        <div class="test">
            // A comment inside a block
            {
                let x = 1;
                // Comment after x
                x
            }
            <span />
        </div>
    };
}
