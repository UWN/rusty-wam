use prolog::and_stack::*;
use prolog::ast::*;

use std::ops::IndexMut;

pub trait CopierTarget
{
    fn source(&self) -> usize;
    fn threshold(&self) -> usize;
    fn push(&mut self, HeapCellValue);
    fn store(&self, Addr) -> Addr;
    fn deref(&self, Addr) -> Addr;
    fn stack(&mut self) -> &mut AndStack;

    // duplicate_term(L1, L2) uses Cheney's algorithm to copy the term
    // at L1 to L2. trail is kept to restore the innards of L1 after
    // it's been copied to L2.
    fn duplicate_term(&mut self, a: Addr) where Self: IndexMut<usize, Output=HeapCellValue>
    {
        let mut trail: Vec<(Ref, HeapCellValue)>= Vec::new();
        let mut scan = self.source();
        let old_h = self.threshold();

        self.push(HeapCellValue::Addr(a));

        while scan < self.threshold() {
            match self[scan].clone() {
                HeapCellValue::NamedStr(..) =>
                    scan += 1,
                HeapCellValue::Addr(a) =>
                    match a.clone() {
                        Addr::Lis(a) => {
                            self[scan] = HeapCellValue::Addr(Addr::Lis(self.threshold()));
                            
                            let hcv = self[a].clone();
                            self.push(hcv);
                            
                            let hcv = self[a+1].clone();
                            self.push(hcv);
                            
                            scan += 1;
                        },
                        Addr::HeapCell(_) | Addr::StackCell(_, _) => {
                            let ra = a;
                            let rd = self.store(self.deref(ra.clone()));

                            match rd.clone() {
                                Addr::HeapCell(hc) if hc >= old_h => {
                                    self[scan] = HeapCellValue::Addr(rd);
                                    scan += 1;
                                },
                                _ if ra == rd => {
                                    self[scan] = HeapCellValue::Addr(Addr::HeapCell(scan));

                                    if let Addr::HeapCell(hc) = ra.clone() {
                                        self[hc] = HeapCellValue::Addr(Addr::HeapCell(scan));
                                        trail.push((Ref::HeapCell(hc),
                                                    HeapCellValue::Addr(Addr::HeapCell(hc))));
                                    } else if let Addr::StackCell(fr, sc) = ra {
                                        self.stack()[fr][sc] = Addr::HeapCell(scan);
                                        trail.push((Ref::StackCell(fr, sc),
                                                    HeapCellValue::Addr(Addr::StackCell(fr, sc))));
                                    }

                                    scan += 1;
                                },
                                _ => self[scan] = HeapCellValue::Addr(rd)
                            };
                        },
                        Addr::Str(s) => {
                            match self[s].clone() {
                                HeapCellValue::NamedStr(arity, name, fixity) => {
                                    let threshold = self.threshold();

                                    self[scan] = HeapCellValue::Addr(Addr::Str(threshold));
                                    self[s] = HeapCellValue::Addr(Addr::Str(threshold));

                                    trail.push((Ref::HeapCell(s),
                                                HeapCellValue::NamedStr(arity, name.clone(), fixity)));

                                    self.push(HeapCellValue::NamedStr(arity, name, fixity));

                                    for i in 0 .. arity {
                                        let hcv = self[s + 1 + i].clone();
                                        self.push(hcv);
                                    }
                                },
                                HeapCellValue::Addr(Addr::Str(o)) =>
                                    self[scan] = HeapCellValue::Addr(Addr::Str(o)),
                                _ => {}
                            };

                            scan += 1;
                        },
                        Addr::Con(_) => scan += 1
                    }
            }
        }

        for (r, hcv) in trail {
            match r {
                Ref::HeapCell(hc) => self[hc] = hcv,
                Ref::StackCell(fr, sc) => self.stack()[fr][sc] = hcv.as_addr(0)
            }
        }
    }
}
