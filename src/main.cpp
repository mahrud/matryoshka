#include <iostream>
#include <memory>
#include <vector>

using namespace std;

using monoid_t = vector<int>;


void peek(const int& x)
{ cout << x; }

void peek(const monoid_t& x)
{ cout << "Monoid("; for (const auto& e: x) peek(e); cout << ")"; }


template <typename T>
struct ring_t {
  explicit ring_t(T x) : inner(make_shared<T>(move(x))) {}
  ~ring_t() = default;

  //  ring_t<ring_t<T>> pile() const { return static_cast<const ring_t<T>>(*this); }
  ring_t<ring_t<T>> pile() const { return ring_t<ring_t<T>>(*this); }
  T peel() const { return *inner; }

  shared_ptr<T> inner;
};

template <typename T>
void peek(const ring_t<T>& x)
{
  cout << "Ring(";
  peek(*x.inner);
  cout << ")";
}

int main()
{
  monoid_t monoid;
  monoid.emplace_back(5);
  monoid.emplace_back(6);

  cout << sizeof(monoid) << ": ";
  peek(monoid);
  cout << endl;

  ring_t ring0(monoid);
  cout << sizeof(ring0) << ": ";
  peek(ring0);
  cout << endl;

  ring_t ring1 = ring0.pile().peel();
  cout << sizeof(ring1) << ": ";
  peek(ring1);
  cout << endl;

  ring_t ring2 = ring1.pile();
  ring_t ring3 = ring2.peel();
  ring_t ring4 = ring3.pile().pile().pile();
  cout << sizeof(ring4) << ": ";
  peek(ring4);
  cout << endl;
}
