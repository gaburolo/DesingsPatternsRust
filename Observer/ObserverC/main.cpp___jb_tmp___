#include <iostream>
#include <iomanip>
#include <queue>
#include <string>
#include <math.h>
#include <ctime>
using namespace std;

//Divide las clases y crea los metodos encargados de actualizar los cambios
class DivObserver
{
    int m_div;
  public:
    DivObserver(int div)
    {
        m_div = div;
    }
    void update(int val)
    {
        cout << val << " div " << m_div << " is " << val / m_div << '\n' ;
    }
};
// Modifica los valores cuando estos cambien
class ModObserver
{
    int m_mod;
  public:
    ModObserver(int mod)
    {
        m_mod = mod;
    }
    void update(int val)
    {
        cout << val << " mod " << m_mod << " is " << val % m_mod << '\n';
    }
};

//Notifica los cambios de los valores
class Subject
{
    int m_value;
    DivObserver m_div_obj;
    ModObserver m_mod_obj;
  public:
    Subject(): m_div_obj(4), m_mod_obj(3){}
    void set_value(int value)
    {
        m_value = value;
        notify();
    }
    void notify()
    {
        m_div_obj.update(m_value);
        m_mod_obj.update(m_value);
    }
};

//Crea el main donde se ejecutará el observer
int main()
{
  Subject subj;
  subj.set_value(14);
}