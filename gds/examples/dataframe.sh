for f in dataframe*.rs; do
  c=${f%.rs}
  echo $c:
  cargo run --example "$c"
done
