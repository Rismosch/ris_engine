#pragma once

namespace ris
{
	class risModule
	{
	public:
		virtual void StartUp() = 0;
		virtual void ShutDown() = 0;
	};
}