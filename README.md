## Input
Where stateful=true uniqueGroupId may be none.

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
