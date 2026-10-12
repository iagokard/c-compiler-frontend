int main(void) {
  int i;
  int soma;
  int n;

  soma = 0;
  n = 10;
  i = 1;

  while (i < n) {
    soma = soma + i;
    i = i + 1;
  }

  if (soma > 20) {
    soma = soma - 20;
  }

  return soma;
}
