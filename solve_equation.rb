[2, 3, 5, 7, 9].permutation.each do |(a, b, c, d, e)|
  puts "#{a} + #{b} * #{c}^2 + #{d}^3 - #{e} == #{a + b * c**2 + d**3 - e}"
end
