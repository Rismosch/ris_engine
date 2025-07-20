GAG
HHA

r/(r+h) = sin(alpha)
d/(r+h) = cos(alpha)

alpha = asin(r/r+h) oder
alpha = acos(d/(r+h))

solve for r:
r/(r+h) = sin(acos(d/(r+h))) oder
d/(r+h) = cos(asin(r/r+h))



r/(r+h) = sin(acos(d/(r+h)))

let:
x = acos(d/(r+h))

then:
sin(x) = sqrt(1-cos^2(x))  // cos^2(x) == cos(x)*cos(x)
sin(acos(d/(r+h))) = sqrt(1-(d/r+h)^2)

r/(r+h) = sqrt(1-(d/r+h)^2)   | ^2
(r/(r+h))^2 = 1-(d/r+h)^2
r^2 / (r+h)^2 = 1 - d^2 / (r+h)^2   | * (r+h)^2
r^2 = (r+h)^2 - d^2
d^2 = (r+h)^2 - r^2
d^2 = (r^2 + 2rh + h^2) - r^2
d^2 = r^2 + 2rh + h^2 - r^2
d^2 = 2rh + h^2
d^2 - h^2 = 2rh
(d^2 - h^2) / 2h = r