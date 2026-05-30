for f in dataset*.rs; do
  c=${f%.rs}
  echo $c:
  cargo run --example "$c"
done
