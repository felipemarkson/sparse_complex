#include <complex>
#include <Eigen/Sparse>
#include <Eigen/SparseLU>
#include <vector>
#include <iostream>

typedef std::complex<double> cdouble;
typedef std::complex<float> cfloat;
typedef Eigen::Triplet<cdouble> T;
typedef Eigen::Triplet<cfloat> T32;

extern "C" struct cmplx64
{
    double re;
    double im;
};

extern "C" struct cmplx32
{
    float re;
    float im;
};

extern "C" void solve_cpp(const cmplx64 *values, const size_t *rows, const size_t *cols, const size_t n_value, cmplx64 *b, const size_t size)
{
    Eigen::SparseMatrix<cdouble> A(size, size);
    A.reserve(n_value);

    std::vector<T> triplets;
    triplets.reserve(n_value);

    Eigen::VectorXcd b_(size);
    Eigen::VectorXcd x(size);

    Eigen::SparseLU< Eigen::SparseMatrix<cdouble> > solver;

    double re, im;
    cdouble value;
    size_t row, col, i;

    for (i = 0; i < n_value; i++)
    {
        re = (values + i)->re;
        im = (values + i)->im;
        value = cdouble(re, im);
        row = *(rows + i);
        col = *(cols + i);
        triplets.push_back(T(row, col, value));
    }

    for (i = 0; i < size; i++)
    {

        re = (b + i)->re;
        im = (b + i)->im;
        value = cdouble(re, im);
        b_[i] = value;
    }

    A.setFromTriplets(triplets.begin(), triplets.end());

    solver.compute(A);
    x = solver.solve(b_);

    for (size_t i = 0; i < size; i++)
    {
        re = x[i].real();
        im = x[i].imag();
        (b + i)->re = re;
        (b + i)->im = im;
    }
}

extern "C" void solve_cpp32(const cmplx32 *values, const size_t *rows, const size_t *cols, const size_t n_value, cmplx32 *b, const size_t size)
{
    Eigen::SparseMatrix<cfloat> A(size, size);
    A.reserve(n_value);

    std::vector<T32> triplets;
    triplets.reserve(n_value);

    Eigen::VectorXcf b_(size);
    Eigen::VectorXcf x(size);

    Eigen::SparseLU< Eigen::SparseMatrix<cfloat> > solver;

    float re, im;
    cfloat value;
    size_t row, col, i;

    for (i = 0; i < n_value; i++)
    {
        re = (values + i)->re;
        im = (values + i)->im;
        value = cfloat(re, im);
        row = *(rows + i);
        col = *(cols + i);
        triplets.push_back(T32(row, col, value));
    }

    for (i = 0; i < size; i++)
    {

        re = (b + i)->re;
        im = (b + i)->im;
        value = cfloat(re, im);
        b_[i] = value;
    }

    A.setFromTriplets(triplets.begin(), triplets.end());

    solver.compute(A);
    x = solver.solve(b_);

    for (size_t i = 0; i < size; i++)
    {
        re = x[i].real();
        im = x[i].imag();
        (b + i)->re = re;
        (b + i)->im = im;
    }
}