(echo "graph G {"; sed 's/-/--/g' $1; echo "}") | dot -Tpng | display - &
