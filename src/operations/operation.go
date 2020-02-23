package operations

type Operation interface {
	Execute() (error, interface{})
}
