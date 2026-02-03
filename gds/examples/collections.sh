for f in collections*.rs; do
  c=${f%.rs}
  echo $c:
  cargo run --example "$c"
done
