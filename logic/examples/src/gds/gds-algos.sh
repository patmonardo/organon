for f in gds*.ts; do
  c=${f}
  echo $c:
  npx tsx "$c"
done
