# Just-flow core

## How to build with docker
Set the version of the official Rust docker that you need. Start from the working directory with Cargo.toml.
```shell
sudo docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/just-flow -w /just-flow rust:1.50.0 cargo build --release
```

## Input
Where stateful=true uniqueGroupId may be none.
```shell
on Linux, rename libyour_module.so to your_module.so.
```

```java
{
	uniqueId: string,
	uniqueGroupId: string,
	stateTransitions: [
	]
}
```


## Config
matchType may be START, END, EVAL

```java
{
	stateful: true,
	stateLossTime: u128,
	flowRoutes: [
		{
			flowRouteName: string,
			flowConditionMatches: [
				{
					routeConditionName: string,
					matchType: "START",
					matchConditionType: "ANY",
					matchConditions: [
						{
							paramName: string,
							paramType: string,
							paramMatch: string
						}
					],
					matchActions: [
						{
							actionName: string,
							actionType: string,
							actionValue: string
						}
					],
					nextAvailableMatches: [
						string,
						string
					]
				}
			]
		}
	]
}						
```
