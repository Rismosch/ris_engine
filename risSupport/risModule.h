#pragma once

namespace risSupport
{
	class risModule
	{
	public:
		virtual void setUp() = 0;
		virtual void shutDown() = 0;
	};
}