anchor_gen::generate_cpi_crate!("idl.json");
anchor_lang::declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

fn main() {
    let swap = typedefs::SwapLeg::Swap {
        swap: typedefs::Swap::Saber,
    };
}
