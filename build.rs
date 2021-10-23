fn main() {
    cc::Build::new().file("lib/ipmiraw.c").compile("ipmiraw");
}
