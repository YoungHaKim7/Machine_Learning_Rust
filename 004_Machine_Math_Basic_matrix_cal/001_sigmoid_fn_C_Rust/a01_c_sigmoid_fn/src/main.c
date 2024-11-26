#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <math.h>

#define M_PI_2        1.57079632679489661923	/* pi/2 */
#define M_PI_2_INV    (1.0/M_PI_2)
#define M_2_SQRTPI    1.12837916709551257390    /* 2/sqrt(pi) */
#define ERF_COEF      (1.0/M_2_SQRTPI)

const int SIZE=100;
const int CYCLES=10000000;

double benchmark(const char* name, double (*fun)(double)) {
	clock_t start, stop;
	double xs[SIZE];
	double t_ns;

	for (int i=0; i<SIZE; i++) {
		xs[i] = rand();
	}

	start = clock();
	for (int repeat=0; repeat<CYCLES; repeat++) {
		for (int i=0; i<SIZE; i++) {
			(*fun)(xs[i]);
		}
	}
	stop = clock();
	t_ns = (stop-start)*1.0e9/CLOCKS_PER_SEC/CYCLES/SIZE;
        printf("%-17s %6.1f ns\n", name, t_ns);
	return t_ns;
}

double with_atan(double x) {
	/* normalized atan */
	return M_PI_2_INV*atan(M_PI_2*x);
}

double with_exp(double x) {
	return 1.0/(1.0 + exp(-x));
}

double with_sqrt(double x) {
	return 1.0/sqrt(1.0 + x*x);
}

double with_erf(double x) {
	return erf(ERF_COEF*x);
}

double with_fabs(double x) {
	return x/(1.0 + fabs(x));
}

int main(int argc, char **argv) {
	benchmark("atan(pi*x/2)*2/pi", with_atan);
	benchmark("atan(x)", atan);
	benchmark("1/(1+exp(-x))", with_exp);
	benchmark("1/sqrt(1+x^2)", with_sqrt);
	benchmark("erf(sqrt(pi)*x/2)", with_erf);
	benchmark("tanh(x)", tanh);
	benchmark("x/(1+|x|)", with_fabs);
}
