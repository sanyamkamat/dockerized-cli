package actions

import "fmt"

type Exec struct {
	Args []string
}

func (a *Exec) Execute() error {
	fmt.Printf("%v", a.Args)
	return nil
}
